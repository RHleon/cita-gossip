//The module defines data structures, 
//which declares message types, 
//and encapsulates logical operations.
//
//
//
//
use::storage::Node;

//The four structure are used for message transmission.
pub struct Short_msg{
    src:Node,
    des:Node,
    data:String,
    flag:bool,
}
pub struct List_msg{
    src:Node,
    des:Node,
    is_request:bool,
    List:Vec<Node>,
}
pub struct Gossip_msg{
    src:Node,
    des:Node,
    data:String,
}
pub struct HeartBeat_msg{
    src:Node,
    des:Node,
    is_alive:bool,
}

//Impl for Short_msg:new()-initialization, send() & receive()
// **Short_msg has been abandoned by now. Maybe it will be used for rror retransmission
impl Short_msg{
    pub fn new(self_node:Node /*the local Node*/) -> Self{
        Short_msg{
            src : self_node,
            des : self_node,
            data: String::from(""),
            flag: false,
        }
    }
    pub fn send(&self,target:&Node){//the destination is provided by Mod-filter
        self.des = target;
        self.flag = false;
        des_ip = super::filter::filter(target);
        super::comm::ShortSend();    
    }
    pub fn receive(&self){
        if self.flag == false {
            let rec_emp_msg = Short_msg{
                src : self.des,
                des : self.src,
                flag : true,
            };
            des_ip = super::filter::filter(rec_emp_msg.des);
            super::comm::ShortSend();    
        }
    }
}

//Impl for List_msg:new()-initialization,send() & receive()
impl List_msg{
    pub fn new(self_node:Node) -> Self{ 
        List_msg{
            src : self_node,
            des : self_node,
            is_request: true,
            List:Vec::new(),
        }
    }
    pub fn send(&mut self,target:&Node,if_request:bool){//the destination is provided by Mod-filter
        self.des = target;
        self.is_request = if_request;
        self.List = super::storage::GetList();
        des_ip = super::filter::filter(target);
        super::comm::ListSend();    //Unfinished
    }
    pub fn receive(&self){
        //change local file through is_request
    }
}

//Impl for Gossip_msg:new()-initialization,send() & receive()
impl Gossip_msg{
    pub fn new(self_node:Node) -> Self{
        Gossip_msg{
            src : self_node,
            des : self_node,
            data: String::from(""),
        }
    }
    pub fn send(&mut self,target:&Node,Data:String){//the destination is provided by Mod-filter
        self.des = target;
        self.data = Data;
        des_ip = super::filter::filter(target);
        super::comm::GossipSend(); 
    }
    pub fn receive(&self){
        //change local file
    }
}

//Impl for HeartBeat_msg:new()-initialization,send() & receive()
impl HeartBeat_msg{
    pub fn new(self_node:Node) -> Self{
        HeartBeat_msg{
            src : self_node,
            des : self_node,
            is_alive: true,
        }
    }
    pub fn send(&mut self,target:&Node,if_alive:bool){//the destination is provided by Mod-filter
        self.des = target;
        self.is_alive = if_alive;
        des_ip = super::filter::filter(target);
        super::comm::HeartBeatSend();  
    }
    pub fn receive(&self){
        //change local file through is_alive
    }
}