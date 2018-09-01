//discovery模块接口
//extern

//全局变量

use::storage::Node;
use::msg::List_msg;
use::filter;

//do not use this struct
	pub struct NetworkMember{
    undefined:Id,
    undefined:Properties,
	}
//

	pub fn ApplyBySeed(){}     //get seed from configuration file
	pub fn Send(){
		let find_msg = msg::List_msg::new(GetSelfNode());
		let des = Node::new();   //new()fn
		des = filter::random_filter();
		find_msg.send(des, true);
}       //send to filter in order to gossip alive


	pub fn GetSelfNode() -> Node{}