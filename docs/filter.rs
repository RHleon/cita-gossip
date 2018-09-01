//filter模块的功能是确定发送目标的id

use::discovery::NetworkMember;
use::storage;
use rand::Rng;

pub fn random_filter() -> NetworkMember{
    let list : Vec<Node> = storage::GetList();
    let mut len:u32 = 0;
    for item in list{
        len += 1;
    }
    let random_number = rand::thread_rng().gen_range(1, len+1);
    //if  { }   need to know the local node.
    let random_target = list.get(random_number)
}
pub fn filter(target:NetworkMember/*发送目标*/) -> iptype{
    let list : Vec<Node> = storage::GetList();
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