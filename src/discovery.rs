//The module is responsible for self discovery function.
//
//Responsible for communicating with configuration files.
//
//calling mDNS module to use mDNS function.
//
//
//

use::storage::Node;
use::msg::List_msg;
use::msg::HeartBeat_msg;
use::filter;

#[macro_use]
extern crate serde_derive;

extern crate toml;
extern crate serde;
extern crate serde_json;
extern crate env_logger;
extern crate mdns;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]

//this structure is used for configuration
pub struct Seed_Node {
    Id: String,
    Ip: String,
    Port: String,
}
impl Seed_Node {
    fn new(_id: &str, _ip: &str, _port: &str) -> Seed_Node {
        Seed_Node {
            Id: _id.to_string(),
            Ip: _ip.to_string(),
            Port: _port.to_string(),
        }
    }
}

// this function get a filename, return a vector of Seed_Nodes from the file.
pub fn get_conli() -> Vec<Seed_Node> {

    let filename = "configuration.toml";
    let pathname = "../data/".to_owned()+filename;
    let path = Path::new(&pathname);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}, exception: {}", path.display(), e)
    };

    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("couldn't read {}, exception: {}", path.display(), e)
    };

    let nodes: Vec<Seed_Node> = serde_json::from_str(&str_val).unwrap();
    
    nodes
}
/*
// this function get a vector of Seed_Nodes and a filename, write this vector into the file.
pub fn write_conli(v: Vec<Seed_Node>) {

    let filename = "configuration.toml";
    let pathname = "../data/".to_owned()+filename;
    let path = Path::new(&pathname);
    let path = Path::new(path);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}, exception: {}", path.display(), e)
    };

    let mut serialized = serde_json::to_string(&v).unwrap();
    match file.write_all(serialized.as_bytes()) {
        Ok(s) => s,
        Err(e) => panic!("couldn't write {}, exception: {}", path.display(), e)
    }
}*/

//this function gets Seed_Node from configuration file 
//and transmits messages to others to change AliveList. 
pub fn ApplyBySeed(){
	let configuration_list: Vec<Seed_Node> = get_conli();
	let mut len:u32 = 0;
    for item in configuration_list{
        len += 1;
    }
    let random_number = rand::thread_rng().gen_range(1, len+1);
    let random_target = configuration_list.get(random_number);

    let alive_add = HeartBeat_msg::new();
    let listrequest_add = List_msg::new();
    alive_add.send(random_target,true);
    listrequest_add.send(random_target,true);

}     

//this function sends List_msg to change AliveList. 
pub fn Send(){
		let find_msg = msg::List_msg::new(GetSelfNode());
		let des = Node::new();   
		des = filter::random_filter();
		find_msg.send(des, true);
} 

// this function calls mDNS module.
pub fn mdns(){
	env_logger::init();

    let responder = mdns::Responder::new().unwrap();
    let _svc = responder.register(
        "_http._tcp".to_owned(),
        "Web Server".to_owned(),
        80,
        &["path=/"],
    );

    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}