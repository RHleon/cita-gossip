////////////////////////////
/// 'unclear': I am not sure the use of the point or its meaning
/// 
/// 
////////////////////////////

extern crate mio;
use mio::tcp::TcpStream;

use std::net::SocketAddr;
use std::str;
use std::io;
use std::collections;
use std::fs;

extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate docopt;
use docopt::Docopt;

extern crate ruslts;
extern crate webpki;
extern crate webpki_roots;
extern crate ct_logs;
extern crate vecio;

mod util; //  /util/mode.rs->WriteVAdapter
use util::WriteVAdapter;  // which glues `rustls::WriteV` trait to `vecio::Rawv`

use rustls::Session;

const CLIENT: mio::Token = mio::Token(0);

// Encapsulations of the TCP-level connections, some connection states, and the underlying TLS-level session.
struct Client {
    socket: TcpStream,
    closing: bool,
    clean_closure: bool,
    tls_session: rustls::ClientSession,
    /*
    ClientSession's inside: 
    pub struct ClientSessionImpl {
    pub config: Arc<ClientConfig>, //保存client端的证书，密钥配置等信息
    pub secrets: Option<SessionSecrets>, //保存握手后的会话密钥
    pub alpn_protocol: Option<String>,
    pub common: SessionCommon, // 完成具体消息传输、加解密等
    pub error: Option<TLSError>,
    pub state: Option<Box<hs::State + Send>>, // 保存握手过程中的交互状态，握手中处理对象都实现State接口
    pub server_cert_chain: CertificatePayload, // 服务端证书链
    */
}

impl Client {
    fn ready(&mut self,
            poll: &mut mio::Poll,
            ev: &mio::Event){
        assert_eq!(ev.token(),CLIENT); //compare the two token

        if ev.readiness().is_readable() {
            self.do_read();
        }
        if ev.readiness().is_writable() {
            self.do_write();
        }
        //the enable of read and write

        if self.is_closed() {
            println!("Connection closed");
            process::exit(if self.clean_closure {0} else {1});
        }
        //the shut down of the client

        self.reregister(poll);  // unclear
    }
}

impl io::Write for Client {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {  // u8: The 8-bit unsigned integer type.
        self.tls_session.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.tls_session.flush()
    }
    //implement write and flush to tlsc_session, 
}

impl io::Read for Client {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.tls_session.read(bytes)
    }
}

impl Client {
    fn new(sock: TcpStream, cfg: Arc<rustls::ClientConfig>) -> Client {
        Client {
            socket: sock,
            closing: false,
            clean_closure: false,
            tls_session: rustls::ClientSession::new(&cfg),  // unclear: the parameter in need for new()
        }
    }

    fn read_source_to_end(&mut self, rd: &mut io::Read) -> Result<usize> {
        let mut buf = Vec::new();
        let len = rd.read_to_end(&mut buf)?;
        self.tls_session.write_all(&buf).unwrap();
        Ok(len)
    }

    fn do_read(&mut self) {
        let rc = self.tls_session.read_tls(&mut self.socket);
        if rc.is_err() {
            println!("TLS read error:{:?}",rc);
            self.closing = true;
            return;
        }

        if rc.unwrap()==0 {
            println!("EOF");
            self.closing = true;
            self.clean_closure = true;
            return;
        }

        let processed = self.tls_session.process_new_packets();
        if processed.is_err() {
            println!("TLS erro: {:?}",processed.unwrap_err());
            self.closing = true;
            return;
        }


        let mut plaintext = Vec::new(); // the plaintext is what the client received
        let rc = self.tls_session.read_to_end(&mut plaintext);
        if !plaintext.is_empty() {
            io::stdout().write_all(&plaintext).unwrap();  // unclear:where to output? is vec satisfied or not
            io::stdout().write_all(&plaintext).unwrap();  // unclear:where to output? is vec satisfied or not?
        }
        if rc.is_err() {
            let err = rc.unwrap_err();
            println!("Plaintext read error: {:?}", err);
            self.clean_closure = err.kind()==io::ErrorKind::ConnectionAborted;
            self.closing = true;
            return;
        }
    }

    fn do_write(&mut self) {
        self.tls_session.writev_tls(&mut WriteVAdapter::new(&mut self.socket)).unwrap();
    }

    fn register(&self,poll: &mut mio::Poll) {
        poll.register(&self.socket,
                    CLIENT,
                    self.ready_interest(),
                    mio::PollOpt::level() | mio::PollOpt::oneshot())
            .unwrap();
    }

    fn reregister(&self,poll: &mut mio::Poll) {
        poll.reregister(&self.socket,
                        CLIENT,
                        self.ready_interest(),
                        mio::PollOpt::level() | mio::PollOpt::oneshot())
            .unwrap();
    }

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

/// loading private key, using rustls way
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
/// unclear: how it works
fn make_config(args: &Args) -> Arc<rustls::ClientConfig> {
    let mut config = rustls::ClientConfig::new();
    config.key_log = Arc::new(rustls::KeyLogFile::new());

    if !args.flag_suite.is_empty() {
        config.ciphersuites = lookup_suites(&args.flag_suite);
    }

    if !args.flag_protover.is_empty() {
        config.versions = lookup_versions(&args.flag_protover);
    }

    if args.flag_cafile.is_some() {
        let cafile = args.flag_cafile.as_ref().unwrap();

        let certfile = fs::File::open(&cafile).expect("Cannot open CA file");
        let mut reader = BufReader::new(certfile);
        config.root_store
            .add_pem_file(&mut reader)
            .unwrap();
    } else {
        config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        config.ct_logs = Some(&ct_logs::LOGS);
    }

    if args.flag_no_tickets {
        config.enable_tickets = false;
    }

    if args.flag_no_sni {
        config.enable_sni = false;
    }

    let persist = Arc::new(PersistCache::new(&args.flag_cache));

    config.set_protocols(&args.flag_proto);
    config.set_persistence(persist);
    config.set_mtu(&args.flag_mtu);

    apply_dangerous_options(args, &mut config);

    if args.flag_auth_key.is_some() || args.flag_auth_certs.is_some() {
        load_key_and_cert(&mut config,
                          args.flag_auth_key
                              .as_ref()
                              .expect("must provide --auth-key with --auth-certs"),
                          args.flag_auth_certs
                              .as_ref()
                              .expect("must provide --auth-certs with --auth-key"));
    }

    Arc::new(config)
}
