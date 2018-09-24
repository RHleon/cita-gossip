//comm 功能：构建fliter到ssl传输层的通道
use::filter::random_filter;
use::filter::filter;

use tlsclient;
use tlsserver;
//fn
//EmptySend
//ListSend
//GossipSend
//HeartBeatSend

pub struct comm{

}
impl comm {
    pub fn EmptySend(des_ip: &iptype) {  // unclear
        let version = env!("CARGO_PKG_NAME").to_string() + ", version: " + env!("CARGO_PKG_VERSION");

        let args: Args = Docopt::new(USAGE)
            .and_then(|d| Ok(d.help(true)))
            .and_then(|d| Ok(d.version(Some(version))))
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        if args.flag_verbose {
            env_logger::Builder::new()
                .parse("trace")
                .init();
        }

        let port = args.flag_port.unwrap_or(443);  // default port
        //let addr = lookup_ipv4(args.arg_hostname.as_str(), port);

        let config = make_config(&args);

        let sock = TcpStream::connect(&des_ip).unwrap();
        let dns_name = webpki::DNSNameRef::try_from_ascii_str(&args.arg_hostname).unwrap();
        let mut tlsclient = TlsClient::new(sock, dns_name, config);

        //here do writing to the stream
        //let mut stdin = io::stdin();
        tlsclient.read_source_to_end().unwrap();//unclear


        let mut poll = mio::Poll::new()
            .unwrap();
        let mut events = mio::Events::with_capacity(32);
        tlsclient.register(&mut poll);
            
        poll.poll(&mut events, None)
            .unwrap();

        for ev in events.iter() {
            tlsclient.ready(&mut poll, &ev);
            }
    }

    pub fn ListSend(des_ip:&iptype, list:&string){  //unclear
        let version = env!("CARGO_PKG_NAME").to_string() + ", version: " + env!("CARGO_PKG_VERSION");

        let args: Args = Docopt::new(USAGE)
            .and_then(|d| Ok(d.help(true)))
            .and_then(|d| Ok(d.version(Some(version))))
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        if args.flag_verbose {
            env_logger::Builder::new()
                .parse("trace")
                .init();
        }

        let port = args.flag_port.unwrap_or(443);  // default port
        //let addr = lookup_ipv4(args.arg_hostname.as_str(), port);

        let config = make_config(&args);

        let sock = TcpStream::connect(&des_ip).unwrap();
        let dns_name = webpki::DNSNameRef::try_from_ascii_str(&args.arg_hostname).unwrap();
        let mut tlsclient = TlsClient::new(sock, dns_name, config);

        //here do writing to the stream
        //let mut stdin = io::stdin();
        tlsclient.read_source_to_end(list).unwrap();//unclear


        let mut poll = mio::Poll::new()
            .unwrap();
        let mut events = mio::Events::with_capacity(32);
        tlsclient.register(&mut poll);
            
        poll.poll(&mut events, None)
            .unwrap();

        for ev in events.iter() {
            tlsclient.ready(&mut poll, &ev);
            }
    }
    pub fn GossipSend(des_ip: &iptype, Data: &String) {
        let version = env!("CARGO_PKG_NAME").to_string() + ", version: " + env!("CARGO_PKG_VERSION");

        let args: Args = Docopt::new(USAGE)
            .and_then(|d| Ok(d.help(true)))
            .and_then(|d| Ok(d.version(Some(version))))
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        if args.flag_verbose {
            env_logger::Builder::new()
                .parse("trace")
                .init();
        }

        let port = args.flag_port.unwrap_or(443);  // default port
        //let addr = lookup_ipv4(args.arg_hostname.as_str(), port);

        let config = make_config(&args);

        let sock = TcpStream::connect(&des_ip).unwrap();
        let dns_name = webpki::DNSNameRef::try_from_ascii_str(&args.arg_hostname).unwrap();
        let mut tlsclient = TlsClient::new(sock, dns_name, config);

        //here do writing to the stream
        //let mut stdin = io::stdin();
        tlsclient.read_source_to_end(Data).unwrap();//unclear


        let mut poll = mio::Poll::new()
            .unwrap();
        let mut events = mio::Events::with_capacity(32);
        tlsclient.register(&mut poll);
            
        poll.poll(&mut events, None)
            .unwrap();

        for ev in events.iter() {
            tlsclient.ready(&mut poll, &ev);
        }
    }

    pub fn HeartBeatSend(des_ip:&iptype, is_alive:&bool) {
        let version = env!("CARGO_PKG_NAME").to_string() + ", version: " + env!("CARGO_PKG_VERSION");

        let args: Args = Docopt::new(USAGE)
            .and_then(|d| Ok(d.help(true)))
            .and_then(|d| Ok(d.version(Some(version))))
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        if args.flag_verbose {
            env_logger::Builder::new()
                .parse("trace")
                .init();
        }

        let port = args.flag_port.unwrap_or(443);  // default port
        //let addr = lookup_ipv4(args.arg_hostname.as_str(), port);

        let config = make_config(&args);

        let sock = TcpStream::connect(&des_ip).unwrap();
        let dns_name = webpki::DNSNameRef::try_from_ascii_str(&args.arg_hostname).unwrap();
        let mut tlsclient = TlsClient::new(sock, dns_name, config);

        //here do writing to the stream
        //let mut stdin = io::stdin();
        tlsclient.read_source_to_end(is_alive).unwrap();//unclear


        let mut poll = mio::Poll::new()
            .unwrap();
        let mut events = mio::Events::with_capacity(32);
        tlsclient.register(&mut poll);
            
        poll.poll(&mut events, None)
            .unwrap();

        for ev in events.iter() {
            tlsclient.ready(&mut poll, &ev);
        }
    }
}

impl comm{
    fn receive(){
        let version = env!("CARGO_PKG_NAME").to_string() + ", version: " + env!("CARGO_PKG_VERSION");

        let args: Args = Docopt::new(USAGE)
            .and_then(|d| Ok(d.help(true)))
            .and_then(|d| Ok(d.version(Some(version))))
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        if args.flag_verbose {
            env_logger::Builder::new()
                .parse("trace")
                .init();
        }

        let mut addr: net::SocketAddr = "0.0.0.0:443".parse().unwrap();  // local
        addr.set_port(args.flag_port.unwrap_or(443));

        let config = make_config(&args);

        let listener = TcpListener::bind(&addr).expect("cannot listen on port");
        let mut poll = mio::Poll::new()
            .unwrap();
        poll.register(&listener,
                    LISTENER,
                    mio::Ready::readable(),
                    mio::PollOpt::level())
            .unwrap();

        let mode = if args.cmd_echo {
            ServerMode::Echo
        } else if args.cmd_http {
            ServerMode::Http  // unclear: could use this
        } else {
            ServerMode::Forward(args.arg_fport.expect("fport required"))
        };

        let mut tlsserv = TlsServer::new(listener, mode, config);

        let mut events = mio::Events::with_capacity(256);
        loop {
            poll.poll(&mut events, None)
                .unwrap();

            for event in events.iter() {
                match event.token() {
                    LISTENER => {
                        if !tlsserv.accept(&mut poll) {
                            break;
                        }
                    }
                    _ => tlsserv.conn_event(&mut poll, &event)
                }
            }
        }
    }
    // unclear : how to identify which msg has been received?
}

