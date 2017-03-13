use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

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

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    return s.finish()
}

fn next_block<'a>(prev_block: &'a Block, data: String) -> Block {
	return Block {
		id: prev_block.id + 1,
		prev_hash: hash(prev_block),
		data: data,
	}
}

fn main() {
	let mut blocks = vec![Block {
		id: 0,
		prev_hash: 0,
		data: "I'm awake!".to_string(),
	}];

	let mut next = next_block(blocks.last().unwrap(), "stop. hammertime.".to_string());
	blocks.push(next);
	let mut next = next_block(blocks.last().unwrap(), "stop. hammertime.".to_string());
	blocks.push(next);

	println!("{:?}", blocks);
}
