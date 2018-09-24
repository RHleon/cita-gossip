
use::storage::Node;
use::msg;
//filter模块的功能是确定发送目标的id
use::storage::Node;
use::storage;
use rand::Rng;

pub fn random_filter() -> Node{
    let list : Vec<Node> = storage::get_list();
    let mut len:u32 = 0;
    for item in list{
        len += 1;
    }
    let random_number = rand::thread_rng().gen_range(1, len+1);
    //if  { }   need to know the local node.
    let random_target = list.get(random_number)
}
pub fn filter(target:NetworkMember/*发送目标*/) -> iptype{
    let list : Vec<Node> = storage::get_list();
    let des_id = NetworkMember.Id;
    let mut flag:bool = false;
    for item in list{
        if item.id == des_id{
            flag = true;
            let ip = item.ip;
        }
    }
    if flag == false {
        println!("No Matched Node!");
    }
    return ip
}
pub fn Spread(){
	//账本更新
	//获取账本时间-String
	let random_des = filter::random_filter();
	let version_msg = msg::Short_msg.new();
	version_msg.send(random_des，Data/*Data为账本时间*/);
}//此为随机发送，单播另设函数
//接收并散布cita想要发布的消息（账本）

pub fn Gossip_Send(des:Node){
	//账本更新
	//获取账本时间-String
	let version_msg = msg::Short_msg.new();
	version_msg.send(des，Data/*Data为账本时间*/);
}

pub fn Gossip(){
    //控制Gossip过程
    /*
    启动监听，
    周期性发送Alive消息（维护集群），
    账本更新触发Gossip消息处理，
    */
}

pub fn Hear(){
	/*对账本时间进行对比，再进行操作
	若本地较新，不予处理；若本地较旧，实例化Gossip消息并发送。更新本地账本。
}
//听闻接收到的消息（账本）
//类似监听的server