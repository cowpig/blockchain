#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate blockchain;
#[macro_use]
extern crate error_chain;


use blockchain::blockchain::{Block, Blockchain, is_valid_chain};
use std::io;

mod errors {
    use serde_json;
    error_chain! {
        foreign_links {
            SerdeJson(serde_json::Error);
        }
        errors {
            InvalidCommand(c: String) {
                description("invalid command")
                display("invalid command {}", c)
            }
        }
    }
}

use errors::*;


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
			"blocks" => {
                self.blocks(msg.data.unwrap());
                send(self.get_blocks());
            },
			"transaction" => send(self.transaction(msg.data.unwrap())),
			other => send(Err(ErrorKind::InvalidCommand(other.to_string()).into()))
		}
	}
    
    /// Updates the blockchain if a longer one is found, returns true if a swap was made
    /// 
    /// Call Node::get_blocks to retrieve the blockchain
	fn blocks(&mut self, data: MsgData) -> Result<bool> {
		let blocks = match data {
			MsgData::Transaction(_) => bail!("need a new blockchain with cmd \"blocks\""),
			MsgData::Blockchain(blocks) => blocks,
        };
        //let new_blocks = resolve(self.blockchain, &mut blocks);
        if self.blockchain.len() < blocks.len() && is_valid_chain(&blocks) {
            self.blockchain = blocks;
            Ok(true)
        } else {
            Ok(false)        
        }
	}

	fn transaction(&self, data: MsgData) -> Result<String> {
	    unimplemented!()
    }

	fn get_blocks(&self) -> Result<String> {
		serde_json::to_string(&self.blockchain).map_err(|e| e.into())
	}
}

fn send(msg: Result<String>) {
    match msg {
       Ok(msg) => println!("{}", msg),
       Err(e) => println!("{}", e)
    }
}	

fn main() {
	let blocks = vec![Block {
		id: 0,
		prev_hash: 0,
		data: "I'm awake!".to_string(),
	}];

	let mut node = Node {
		blockchain: blocks,
	};
	
	loop {
		let mut buffer = String::new();
		match io::stdin().read_line(&mut buffer) {
			Ok(n) => {
				println!("{} bytes read", n);
			}
			Err(error) => println!("error: {}", error),
		}
		let data = match serde_json::from_str(buffer.as_str()) {
			Ok(val) => node.handle(val),
			Err(_) => println!("msg should take the form {{\"cmd\": \"blocks|transaction|get_blocks\", \"data\": <data>}}")
		};
		
	}
}
