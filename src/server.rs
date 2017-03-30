#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate nix;
extern crate blockchain;
extern crate time;

use std::collections::hash_map::{HashMap, Entry};
// use std::io;
use std::env;
use std::thread::sleep;
use std::time::{Duration};
use nix::unistd::getpid;
use time::{now};

use blockchain::blockchain::{Block, Blockchain};
use blockchain::wordvote::{VoteChain};
use blockchain::io_queue::{get_redisconn, redis_pop, redis_push};

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
	max_remainder: u8,

	seconds_per_vote: i64,

	last_update: i64,
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
			self.current_votes = HashMap::new();
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
		return now().to_timespec().sec - self.seconds_per_vote > self.last_update;
	}

	fn choose_next_word(&mut self) {
		if self.current_votes.keys().len() < 1 {
			return;
		}

		let mut next_word = "".to_string();
		let mut most_votes = 0;
		for v in self.current_votes.keys() {
			let wv = self.current_votes.get(v).unwrap();
			if wv.votes.len() > most_votes {
				most_votes = wv.votes.len();
				next_word = wv.word.clone();
			}
		}
		{
			let next_data = self.current_votes.get(&next_word).unwrap();
			self.blockchain.extend(next_data.clone());
		}

		self.current_votes = HashMap::new();
	}
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

		seconds_per_vote: 15,

		last_update: now().to_timespec().sec,
	};

	let args: Vec<String> = env::args().collect();
    let name = if args.len() > 1 {
        args[1].clone()
    } else {
        getpid().to_string()
    };

	let redisq = get_redisconn().unwrap();
    let recv_key = format!("node-{pid}-recv", pid=name);
    let send_key = format!("node-{pid}-send", pid=name);

    println!("Listening on redis://127.0.0.1/0 keys:{} & {}", recv_key, send_key);

	redis_push(&redisq, &send_key, "{\"cmd\":\"start\",\"args\":\"\"}".to_string()).unwrap();
	loop {
	    match redis_pop(&redisq, &recv_key) {
	        Err(err) => {
	            println!("got redis err: {}, retrying in 3 sec...", err);
	            sleep(Duration::new(3, 0));
	        },
	        Ok(ref val) if val == "" => {
	        	if node.time_to_update() {
	        		node.choose_next_word();
	        		node.last_update = now().to_timespec().sec;
	        		redis_push(&redisq, &send_key, node.get_blocks()).unwrap();
	        	} else {
		            sleep(Duration::from_millis(100));
		        }
	        },
	        Ok(input) => {
	            println!("[IN]:  {}", input);
	            let result = match serde_json::from_str(&input) {
	            	Ok(input) => node.response(input),
	            	Err(_) => "msg should take the form {\"cmd\": \"[get|send]_[votes|blocks]\", \"data\": <Blocks|Votes>}".to_string(),
	            };
	            println!("[OUT]: {}", result);
	            redis_push(&redisq, &send_key, format!("{{\"out\":\"{}\"}}", result)).unwrap();
	        }
	    }
	}
}
