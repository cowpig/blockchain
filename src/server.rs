#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate blockchain;

use blockchain::blockchain::{Block, Blockchain, replaces};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
enum MsgData {
	Transaction(String),
	Blockchain(Blockchain),
}

#[derive(Serialize, Deserialize, Debug)]
struct MsgStruct {
	cmd: String,
	data: Option<MsgData>,
}

struct Node {
	blockchain: Blockchain
}

impl Node {
	fn handle (&mut self, msg: MsgStruct) {
		match msg.cmd.as_ref() {
			"get_blocks" => send(self.get_blocks()),
			"blocks" => send(self.blocks(msg.data.unwrap())),
			"transaction" => send(self.transaction(msg.data.unwrap())),
			_ => send("error".to_string())
		}
	}

	fn blocks(&mut self, data: MsgData) -> String {
		match data {
			MsgData::Transaction(_) => "need a new blockchain with cmd \"blocks\"".to_string(),
			MsgData::Blockchain(blocks) => {
				if replaces(&self.blockchain, &blocks) {
					self.blockchain = blocks;
				}
				return serde_json::to_string(&self.blockchain).unwrap();
			}
		}
	}

	fn transaction(&self, data: MsgData) -> String {
		return "TODO".to_string()
	}

	fn get_blocks(&self) -> String {
		return serde_json::to_string(&self.blockchain).unwrap();
	}

}

fn send(msg: String) {
	println!("{}", msg)
}	

fn main() {

	let mut node = Node {
		blockchain: vec![Block {
			id: 0,
			prev_hash: 0,
			data: "I'm awake!".to_string(),
		}]
	};
	
	loop {
		let mut buffer = String::new();
		match io::stdin().read_line(&mut buffer) {
			Ok(n) => {
				println!("{} bytes read", n);
			}
			Err(error) => println!("error: {}", error),
		}
		match serde_json::from_str(buffer.as_str()) {
			Ok(val) => node.handle(val),
			Err(error) => println!("msg should take the form {{\"cmd\": \"blocks|transaction|get_blocks\", \"data\": <data>}}")
		};
		
	}
}
