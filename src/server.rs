extern crate rustc_serialize;
extern crate blockchain;
extern crate serde_derive;
extern crate serde_json;

use blockchain::blockchain::{Block, Blockchain};
use std::io;

enum MsgData {
	Transaction(String),
	Blockchain(Blockchain),
}

#[derive(Serialize, Deserialize)]
struct MsgStruct {
	cmd: String,
	data: Option<MsgData>,
}

struct Node {
	blockchain: Blockchain
}

impl Node {
	fn handle (&self, msg: MsgStruct) {
		match msg.cmd.as_ref() {
			"get_blocks" => send(self.get_blocks()),
			"blocks" => send(self.blocks(msg.data.unwrap())),
			"transaction" => send(self.transaction(msg.data.unwrap())),
			_ => send("error".to_string())
		}
	}

	fn blocks(&self, block_json: &Json) -> String {
		return "TODO".to_string()
	}

	fn transaction(&self, block_json: &Json) -> String {
		return "TODO".to_string()
	}

	fn get_blocks(&self) -> String {
		return "TODO".to_string() //to_json(self.blockchain)
	}
}

fn send(msg: String) {
	println!("{}", msg)
}	

fn main() {
	let mut blocks = vec![Block {
		id: 0,
		prev_hash: 0,
		data: "I'm awake!".to_string(),
	}];

	let node = Node {
		blockchain: blocks,
	};
	
	loop {
		let mut buffer = String::new();
		match io::stdin().read_line(&mut buffer) {
		    Ok(n) => {
		        println!("{} bytes read", n);
		        println!("{}", buffer);
		    }
		    Err(error) => println!("error: {}", error),
		}
		// node.handle(parse(buffer));
	}
}
