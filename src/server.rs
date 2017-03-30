#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate blockchain;

use std::collections::hash_map::{HashMap, Entry};
use std::io;

use blockchain::blockchain::{Block, Blockchain};
use blockchain::wordvote::{VoteChain};


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum MsgData {
	Blockchain(Blockchain),
	VoteChain(VoteChain),
}

#[derive(Serialize, Deserialize, Debug)]
struct MsgStruct {
	cmd: String,
	data: Option<MsgData>,
}

struct Node {
	blockchain: Blockchain,
	current_votes: HashMap<String, VoteChain>,
	
	// parameters for PoW function
	n_bytes: usize, 
	max_remainder: u8
}

impl Node {
	fn response(&mut self, msg: MsgStruct) -> String {
		match msg.cmd.as_ref() {
			"get_story" => self.get_story(),
			"get_blocks" => self.get_blocks(),
			"get_votes" => self.get_votes(),
			"set_blocks" => match msg.data.unwrap() {
				MsgData::Blockchain(_) => "need a votechain with cmd 'set_votes'".to_string(),
				MsgData::VoteChain(vc) => self.set_votes(vc)
			},
			"set_votes" => match msg.data.unwrap() {
				MsgData::VoteChain(_) => "need a new blockchain with cmd 'set_blocks'".to_string(),
				MsgData::Blockchain(blocks) => self.set_blocks(blocks)
			},
			_ => "error: unknown cmd".to_string()
		}
	}

	fn get_story(&self) -> String {
		let mut story = "".to_string();
		for block in self.blockchain.blocks.iter() {
			story += &block.data.word;
		}
		return story;
	}

	fn get_blocks(&self) -> String {
		return serde_json::to_string(&self.blockchain).unwrap();
	}

	fn set_blocks(&mut self, blocks: Blockchain) -> String {
		if self.blockchain.replaced_by(&blocks) {
			self.blockchain = blocks;
			return "accept".to_string()
		}
		return "reject".to_string()
	}

	fn get_votes(&self) -> String {
		return serde_json::to_string(&self.current_votes).unwrap();
	}

	fn set_votes(&mut self, vc: VoteChain) -> String {
		if !self.is_valid_votechain(&vc) {
			return "invalid".to_string();
		}

		let word = vc.word.clone();
		match self.current_votes.entry(word) {
			Entry::Occupied(ref curr_vc) if !curr_vc.get().replaced_by(&vc, self.n_bytes, self.max_remainder) => {
				return "reject".to_string();
			},
			Entry::Occupied(mut entry) => {
				entry.insert(vc);
				return "accept".to_string();
			},
			Entry::Vacant(entry) => {
				entry.insert(vc);
				return "accept".to_string();
			}
		}
	}

	fn is_valid_votechain(&self, votechain: &VoteChain) -> bool {
		return votechain.is_valid(self.n_bytes, self.max_remainder);
	}

	fn time_to_update(&self) -> bool {
		// every n_seconds seconds return true
		unimplemented!();
		return false
	}

	fn update(&self) {
		// choose the votechain with the most votes
		// broadcast it
		unimplemented!();
	}
}

fn send(msg: String) {
	println!("{}", msg);
}	

fn main() {
	let mut node = Node {
		blockchain: Blockchain {
			blocks: vec![Block {
				id: 0,
				prev_hash: "".to_string(),
				data: VoteChain {
					word: "".to_string(),
					votes: vec![],
				},
			}]
		},
		current_votes: HashMap::new(),
		n_bytes: 2,
		max_remainder: 5,
	};
	
	loop {
		let mut buffer = String::new();
		match io::stdin().read_line(&mut buffer) {
			Ok(n) => {
				send(format!("{} bytes read", n));
			}
			Err(error) => send(format!("error: {}", error)),
		}
		match serde_json::from_str(buffer.as_str()) {
			Ok(val) => send(node.response(val)),
			Err(error) => {
				send(format!("{:?}", error));
				send("msg should take the form {{\"cmd\": \"[get|send]_[votes|blocks]\", \"data\": <Blocks|Votes>}}".to_string());
			}
		};
	}
}
