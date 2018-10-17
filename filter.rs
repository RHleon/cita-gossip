
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
	let version_msg = msg::Short_msg::new();
	version_msg.send(random_des，Data/*Data为账本时间*/);
}//此为随机发送，单播另设函数
//接收并散布cita想要发布的消息（账本）

pub fn Gossip_Send(des:Node){
	//账本更新
	//获取账本时间-String
	let version_msg = msg::Short_msg::new();
	version_msg.send(des，Data/*Data为账本时间*/);
}

pub fn Gossip(){
    //控制Gossip过程
    /*
    启动监听，
    周期性发送Alive消息（维护集群），
    账本更新触发Gossip消息处理，
    */
   let interval = Duration::milliseconds(1000);
    let mut timer = Timer:new().unwrap();
    let oneshot: Receiver<()> = timer.oneshot(interval);

    oneshot.recv();

    timer::sleep(interval);

    let metronome: Receiver<()> = timer.periodic(interval);

    for i in iter::range_step(5i, 0, -1) {
        metronome.recv();
        let alive_msg = msg::List_msg::new();
        let op = random_filter();
        alive_msg.send(op,ture);
    }
    metronome.recv();
}