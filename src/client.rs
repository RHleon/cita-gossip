use std::sync::{Arc, Mutex};
use std::process;

extern crate mio;
use mio::tcp::TcpStream;

use std::net::SocketAddr;
use std::str;
use std::io;
use std::fs;
use std::collections;
use std::io::{Read, Write, BufReader};

extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate docopt;
use docopt::Docopt;

extern crate rustls;
extern crate webpki;
extern crate webpki_roots;
extern crate ct_logs;
extern crate vecio;

mod util;
use util::WriteVAdapter;

use rustls::Session;

const CLIENT: mio::Token = mio::Token(0);

/// This encapsulates the TCP-level connection, some connection
/// state, and the underlying TLS-level session.
struct TlsClient {
    socket: TcpStream,
    closing: bool,
    clean_closure: bool,
    tls_session: rustls::ClientSession,
}

impl TlsClient {
    fn ready(&mut self,
             poll: &mut mio::Poll,
             ev: &mio::Event) {
        assert_eq!(ev.token(), CLIENT);

        if ev.readiness().is_readable() {
            self.do_read();
        }

        if ev.readiness().is_writable() {
            self.do_write();
        }

        if self.is_closed() {
            println!("Connection closed");
            process::exit(if self.clean_closure { 0 } else { 1 });
        }

        self.reregister(poll);
    }
}

/// We implement `io::Write` and pass through to the TLS session
impl io::Write for TlsClient {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.tls_session.write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tls_session.flush()
    }
}

impl io::Read for TlsClient {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.tls_session.read(bytes)
    }
}

impl TlsClient {
    fn new(sock: TcpStream, hostname: webpki::DNSNameRef, cfg: Arc<rustls::ClientConfig>) -> TlsClient {
        TlsClient {
            socket: sock,
            closing: false,
            clean_closure: false,
            tls_session: rustls::ClientSession::new(&cfg, hostname),
        }
    }

    fn read_source_to_end(&mut self, rd: &mut io::Read) -> io::Result<usize> {
        let mut buf = Vec::new();
        let len = rd.read_to_end(&mut buf)?;
        self.tls_session.write_all(&buf).unwrap();
        Ok(len)
    }

    /// We're ready to do a read.
    fn do_read(&mut self) {
        // Read TLS data.  This fails if the underlying TCP connection
        // is broken.
        let rc = self.tls_session.read_tls(&mut self.socket);
        if rc.is_err() {
            println!("TLS read error: {:?}", rc);
            self.closing = true;
            return;
        }

        // If we're ready but there's no data: EOF.
        if rc.unwrap() == 0 {
            println!("EOF");
            self.closing = true;
            self.clean_closure = true;
            return;
        }

        // Reading some TLS data might have yielded new TLS
        // messages to process.  Errors from this indicate
        // TLS protocol problems and are fatal.
        let processed = self.tls_session.process_new_packets();
        if processed.is_err() {
            println!("TLS error: {:?}", processed.unwrap_err());
            self.closing = true;
            return;
        }

        // Having read some TLS data, and processed any new messages,
        // we might have new plaintext as a result.
        //
        // Read it and then write it to stdout.
        let mut plaintext = Vec::new();
        let rc = self.tls_session.read_to_end(&mut plaintext);
        if !plaintext.is_empty() {
            io::stdout().write_all(&plaintext).unwrap();
        }

        // If that fails, the peer might have started a clean TLS-level
        // session closure.
        if rc.is_err() {
            let err = rc.unwrap_err();
            println!("Plaintext read error: {:?}", err);
            self.clean_closure = err.kind() == io::ErrorKind::ConnectionAborted;
            self.closing = true;
            return;
        }
    }

    fn do_write(&mut self) {
        self.tls_session.writev_tls(&mut WriteVAdapter::new(&mut self.socket)).unwrap();
    }

    fn register(&self, poll: &mut mio::Poll) {
        poll.register(&self.socket,
                      CLIENT,
                      self.ready_interest(),
                      mio::PollOpt::level() | mio::PollOpt::oneshot())
            .unwrap();
    }

    fn reregister(&self, poll: &mut mio::Poll) {
        poll.reregister(&self.socket,
                        CLIENT,
                        self.ready_interest(),
                        mio::PollOpt::level() | mio::PollOpt::oneshot())
            .unwrap();
    }

    // Use wants_read/wants_write to register for different mio-level
    // IO readiness events.
    fn ready_interest(&self) -> mio::Ready {
        let rd = self.tls_session.wants_read();
        let wr = self.tls_session.wants_write();

        if rd && wr {
            mio::Ready::readable() | mio::Ready::writable()
        } else if wr {
            mio::Ready::writable()
        } else {
            mio::Ready::readable()
        }
    }

    fn is_closed(&self) -> bool {
        self.closing
    }
}

/// This is an example cache for client session data.
/// It optionally dumps cached data to a file, but otherwise
/// is just in-memory.
///
/// Note that the contents of such a file are extremely sensitive.
/// Don't write this stuff to disk in production code.
struct PersistCache {
    cache: Mutex<collections::HashMap<Vec<u8>, Vec<u8>>>,
    filename: Option<String>,
}

impl PersistCache {
    /// Make a new cache.  If filename is Some, load the cache
    /// from it and flush changes back to that file.
    fn new(filename: &Option<String>) -> PersistCache {
        let cache = PersistCache {
            cache: Mutex::new(collections::HashMap::new()),
            filename: filename.clone(),
        };
        if cache.filename.is_some() {
            cache.load();
        }
        cache
    }

    /// If we have a filename, save the cache contents to it.
    fn save(&self) {
        use rustls::internal::msgs::codec::Codec;
        use rustls::internal::msgs::base::PayloadU16;

        if self.filename.is_none() {
            return;
        }

        let mut file = fs::File::create(self.filename.as_ref().unwrap())
            .expect("cannot open cache file");

        for (key, val) in self.cache.lock().unwrap().iter() {
            let mut item = Vec::new();
            let key_pl = PayloadU16::new(key.clone());
            let val_pl = PayloadU16::new(val.clone());
            key_pl.encode(&mut item);
            val_pl.encode(&mut item);
            file.write_all(&item).unwrap();
        }
    }

    /// We have a filename, so replace the cache contents from it.
    fn load(&self) {
        use rustls::internal::msgs::codec::{Codec, Reader};
        use rustls::internal::msgs::base::PayloadU16;

        let mut file = match fs::File::open(self.filename.as_ref().unwrap()) {
            Ok(f) => f,
            Err(_) => return,
        };
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let mut cache = self.cache.lock()
            .unwrap();
        cache.clear();
        let mut rd = Reader::init(&data);

        while rd.any_left() {
            let key_pl = PayloadU16::read(&mut rd).unwrap();
            let val_pl = PayloadU16::read(&mut rd).unwrap();
            cache.insert(key_pl.0, val_pl.0);
        }
    }
}

impl rustls::StoresClientSessions for PersistCache {
    /// put: insert into in-memory cache, and perhaps persist to disk.
    fn put(&self, key: Vec<u8>, value: Vec<u8>) -> bool {
        self.cache.lock()
            .unwrap()
            .insert(key, value);
        self.save();
        true
    }

    /// get: from in-memory cache
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.cache.lock()
            .unwrap()
            .get(key).cloned()
    }
}

const USAGE: &'static str = "
Connects to the TLS server at hostname:PORT.  The default PORT
is 443.  By default, this reads a request from stdin (to EOF)
before making the connection.  --http replaces this with a
basic HTTP GET request for /.

If --cafile is not supplied, a built-in set of CA certificates
are used from the webpki-roots crate.

Usage:
  tlsclient [options] [--suite SUITE ...] [--proto PROTO ...] <hostname>
  tlsclient (--version | -v)
  tlsclient (--help | -h)

Options:
    -p, --port PORT     Connect to PORT [default: 443].
    --http              Send a basic HTTP GET request for /.
    --cafile CAFILE     Read root certificates from CAFILE.
    --auth-key KEY      Read client authentication key from KEY.
    --auth-certs CERTS  Read client authentication certificates from CERTS.
                        CERTS must match up with KEY.
    --protover VERSION  Disable default TLS version list, and use
                        VERSION instead.  May be used multiple times.
    --suite SUITE       Disable default cipher suite list, and use
                        SUITE instead.  May be used multiple times.
    --proto PROTOCOL    Send ALPN extension containing PROTOCOL.
                        May be used multiple times to offer several protocols.
    --cache CACHE       Save session cache to file CACHE.
    --no-tickets        Disable session ticket support.
    --no-sni            Disable server name indication support.
    --insecure          Disable certificate verification.
    --verbose           Emit log output.
    --mtu MTU           Limit outgoing messages to MTU bytes.
    --version, -v       Show tool version.
    --help, -h          Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_port: Option<u16>,
    flag_http: bool,
    flag_verbose: bool,
    flag_protover: Vec<String>,
    flag_suite: Vec<String>,
    flag_proto: Vec<String>,
    flag_mtu: Option<usize>,
    flag_cafile: Option<String>,
    flag_cache: Option<String>,
    flag_no_tickets: bool,
    flag_no_sni: bool,
    flag_insecure: bool,
    flag_auth_key: Option<String>,
    flag_auth_certs: Option<String>,
    arg_hostname: String,
}

// TODO: um, well, it turns out that openssl s_client/s_server
// that we use for testing doesn't do ipv6.  So we can't actually
// test ipv6 and hence kill this.
fn lookup_ipv4(host: &str, port: u16) -> SocketAddr {
    use std::net::ToSocketAddrs;

    let addrs = (host, port).to_socket_addrs().unwrap();
    for addr in addrs {
        if let SocketAddr::V4(_) = addr {
            return addr;
        }
    }

    unreachable!("Cannot lookup address");
}

/// Find a ciphersuite with the given name
fn find_suite(name: &str) -> Option<&'static rustls::SupportedCipherSuite> {
    for suite in &rustls::ALL_CIPHERSUITES {
        let sname = format!("{:?}", suite.suite).to_lowercase();

        if sname == name.to_string().to_lowercase() {
            return Some(suite);
        }
    }

    None
}

/// Make a vector of ciphersuites named in `suites`
fn lookup_suites(suites: &[String]) -> Vec<&'static rustls::SupportedCipherSuite> {
    let mut out = Vec::new();

    for csname in suites {
        let scs = find_suite(csname);
        match scs {
            Some(s) => out.push(s),
            None => panic!("cannot look up ciphersuite '{}'", csname),
        }
    }

    out
}

/// Make a vector of protocol versions named in `versions`
fn lookup_versions(versions: &[String]) -> Vec<rustls::ProtocolVersion> {
    let mut out = Vec::new();

    for vname in versions {
        let version = match vname.as_ref() {
            "1.2" => rustls::ProtocolVersion::TLSv1_2,
            "1.3" => rustls::ProtocolVersion::TLSv1_3,
            _ => panic!("cannot look up version '{}', valid are '1.2' and '1.3'", vname),
        };
        out.push(version);
    }

    out
}

fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls::internal::pemfile::certs(&mut reader).unwrap()
}

fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let keyfile = fs::File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);
    let keys = rustls::internal::pemfile::rsa_private_keys(&mut reader).unwrap();
    assert!(keys.len() == 1);
    keys[0].clone()
}

fn load_key_and_cert(config: &mut rustls::ClientConfig, keyfile: &str, certsfile: &str) {
    let certs = load_certs(certsfile);
    let privkey = load_private_key(keyfile);

    config.set_single_client_cert(certs, privkey);
}

#[cfg(feature = "dangerous_configuration")]
mod danger {
    use super::rustls;
    use webpki;

    pub struct NoCertificateVerification {}

    impl rustls::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(&self,
                              _roots: &rustls::RootCertStore,
                              _presented_certs: &[rustls::Certificate],
                              _dns_name: webpki::DNSNameRef,
                              _ocsp: &[u8]) -> Result<rustls::ServerCertVerified, rustls::TLSError> {
            Ok(rustls::ServerCertVerified::assertion())
        }
    }
}

#[cfg(feature = "dangerous_configuration")]
fn apply_dangerous_options(args: &Args, cfg: &mut rustls::ClientConfig) {
    if args.flag_insecure {
        cfg
            .dangerous()
            .set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));
    }
}

#[cfg(not(feature = "dangerous_configuration"))]
fn apply_dangerous_options(args: &Args, _: &mut rustls::ClientConfig) {
    if args.flag_insecure {
        panic!("This build does not support --insecure.");
    }
}

/// Build a `ClientConfig` from our arguments
/// here we can change the source of Args, setting fixed file path to constrains CA
fn make_config() -> Arc<rustls::ClientConfig> {
    let mut config = rustls::ClientConfig::new();
    config.key_log = Arc::new(rustls::KeyLogFile::new());

    //let cafile = args.flag_cafile.as_ref().unwrap();// this is the parsing way, the fixed approach can be adjusted

    let cafile = "test-ca/rsa/ca.cert";//we fix the path of the cert

    let certfile = fs::File::open(&cafile).expect("Cannot open CA file");
    let mut reader = BufReader::new(certfile);
    config.root_store
        .add_pem_file(&mut reader)
        .unwrap();
    
    apply_dangerous_options(args, &mut config);  // the function that apply impl dangerous
    Arc::new(config)
}

/// Parse some arguments, then make a TLS client connection
/// somewhere.
fn send(hostname:&string, ip:&string, port:&string, data:&string ) {

    let port = port.unwrap_or(443);  // port could be change as the para port if given
    let addr = ip;

    let config = make_config();  // no need to input para 

    let sock = TcpStream::connect(&addr).unwrap();
    let dns_name = webpki::DNSNameRef::try_from_ascii_str(&hostname).unwrap();
    let mut tlsclient = TlsClient::new(sock, dns_name, config);

    tlsclient.read_source_to_end(&data).unwrap();
    
    //here new a eventpoll,register the event with tlsclient
    let mut poll = mio::Poll::new()
        .unwrap();
    let mut events = mio::Events::with_capacity(32);
    tlsclient.register(&mut poll);

    loop {
        poll.poll(&mut events, None)
            .unwrap();

        for ev in events.iter() {
            tlsclient.ready(&mut poll, &ev);
        }
    }
}
