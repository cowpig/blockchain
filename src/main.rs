use std::hash::{Hash, Hasher, SipHasher};

#[derive(Debug)]
struct Block {
	id: u64,
	prev_hash: u64,
	data: String,
}

impl Hash for Block {
	fn hash<H:Hasher>(&self, state:&mut H) {
		self.id.hash(state);
		self.prev_hash.hash(state);
		self.data.hash(state);
	}
}

fn new_block(prev_block: Block, data: String) -> Block {
	return Block {
		id: prev_block.id + 1,
		prev_hash: 0, //TODO
		data: data,
	}
}

fn main() {
	let genesis_block = Block {
		id: 0,
		prev_hash: 0,
		data: "I'm awake!".to_string(),
	};

	let first_block = new_block(genesis_block, "bojangles".to_string());
	println!("{:?}", first_block);
}
