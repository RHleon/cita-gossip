
use::discovery::NetworkMember;
use::storage::Node;

pub struct Empty_msg{
    src:NetworkMember,
    des:NetworkMember,
    flag:bool,
}
pub struct List_msg{
    src:NetworkMember,
    des:NetworkMember,
    is_request:bool,
    List:Vec<Node>,
}
pub struct Gossip_msg{
    src:NetworkMember,
    des:NetworkMember,
    data:String,
}
pub struct HeartBeat_msg{
    src:NetworkMember,
    des:NetworkMember,
    is_alive:bool,
}

impl Empty_msg{
    pub fn new(self_node:NetworkMember/*the local NetworkMember*/) -> Self{
        Empty_msg{
            src : self_node,
            des : self_node,
            flag: false,
        }
    }
    pub fn send(&self,target:&NetworkMember){//the destination is provided by Mod-filter
        self.des = target;
        self.flag = false;
        des_ip = super::filter::filter(target);
        super::comm::EmptySend();    //Unfinished
    }
    pub fn receive(&self){
        if self.flag == false {
            let rec_emp_msg = Empty_msg{
                src : self.des,
                des : self.src,
                flag : true,
            };
            des_ip = super::filter::filter(rec_emp_msg.des);
            super::comm::EmptySend();    //Unfinished
        }
    }
}
impl List_msg{
    pub fn new(self_node:NetworkMember/*the local NetworkMember*/) -> Self{
        List_msg{
            src : self_node,
            des : self_node,
            is_request: true,
            List:Vec::new(),
        }
    }
    pub fn send(&mut self,target:&NetworkMember,if_request:bool){//the destination is provided by Mod-filter
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
impl Gossip_msg{
    pub fn new(self_node:NetworkMember/*the local NetworkMember*/) -> Self{
        Gossip_msg{
            src : self_node,
            des : self_node,
            data: String::from(""),
        }
    }
    pub fn send(&mut self,target:&NetworkMember,Data:String){//the destination is provided by Mod-filter
        self.des = target;
        self.data = Data;
        des_ip = super::filter::filter(target);
        super::comm::GossipSend();    //Unfinished
    }
    pub fn receive(&self){
        //change local file
    }
}
impl HeartBeat_msg{
    pub fn new(self_node:NetworkMember/*the local NetworkMember*/) -> Self{
        HeartBeat_msg{
            src : self_node,
            des : self_node,
            is_alive: true,
        }
    }
    pub fn send(&mut self,target:&NetworkMember,if_alive:bool){//the destination is provided by Mod-filter
        self.des = target;
        self.is_alive = if_alive;
        des_ip = super::filter::filter(target);
        super::comm::HeartBeatSend();    //Unfinished
    }
    pub fn receive(&self){
        //change local file through is_alive
    }
}



//短消息问题：在此处设置消息还是comm？