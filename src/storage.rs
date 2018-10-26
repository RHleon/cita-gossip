//This module is responsible for static file processing.
//
//
//Declares rules for modifying local data and provides data interfaces.
//
//
//mainly relating to self discovery function and cluster maintenance.
#[macro_use]
extern crate serde_derive;

extern crate toml;
extern crate serde;
extern crate serde_json;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    id: String,
    ip: String,
    timestamp: String,
}
impl Node {
    fn new(_id: &str, _ip: &str, _timestamp: &str) -> Node {
        Node {
            id: _id.to_string(),
            ip: _ip.to_string(),
            timestamp: _timestamp.to_string(),
        }
    }
}


// this function get a filename, return a vector of Nodes from the file.
pub fn get_list() -> Vec<Node> {

    /*open the file; exception handling*/
    let filename = "data.toml";
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

    let nodes: Vec<Node> = serde_json::from_str(&str_val).unwrap();
    
    nodes
}


// this function get a vector of Nodes and a filename, write this vector into the file.
pub fn write_list(v: Vec<Node>) {

    /*open the file; exception handling*/
    let filename = "data.toml";
    let pathname = "../data/".to_owned()+filename;
    let path = Path::new(&pathname);
    let path = Path::new(path);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}, exception: {}", path.display(), e)
    };

    /*serialize and write to file*/
    let mut serialized = serde_json::to_string(&v).unwrap();
    match file.write_all(serialized.as_bytes()) {
        Ok(s) => s,
        Err(e) => panic!("couldn't write {}, exception: {}", path.display(), e)
    }
}


// this function get a Node and a filename, change the Node's timestamp in the file into "***".
pub fn dead_deal(v: Node) {
    /*open the file; exception handling*/
    let filename = "data.toml";
    let pathname = "../data/".to_owned()+filename;
    let path = Path::new(&pathname);
    let path = Path::new(path);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}, exception: {}", path.display(), e)
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("couldn't read {}, exception: {}", path.display(), e)
    };

    /* find the dead node via id and change its timestamp into '***' */
    let mut nodes: Vec<Node> = serde_json::from_str(&str_val).unwrap();
    let mut new_nodes: Vec<Node> = Vec::new();
    for mut node in nodes {
        if node.id == v.id {
            let mut new_node: Node = Node::new(&node.id.clone(), &node.ip.clone(), &"***".to_string());
            new_nodes.push(new_node);
        }
        else {
            new_nodes.push(node);
        }
    }

    /* write the new Vec<Node> into file */
    file = match File::create(path) {
            Ok(file) => file,
            Err(e) => panic!("couldn't open {}, exception: {}", path.display(), e)
        };
    let mut serialized = serde_json::to_string(&new_nodes).unwrap();
    match file.write_all(serialized.as_bytes()) {
        Ok(s) => s,
        Err(e) => panic!("couldn't write {}, exception: {}", path.display(), e)
    }
}