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

fn next_block(prev_block: & Block, data: String) -> Block {
	return Block {
		id: prev_block.id + 1,
		prev_hash: hash(prev_block),
		data: data,
	}
}

fn is_valid_block(prev_block: & Block, new_block: & Block) -> bool {
	return prev_block.id + 1 == new_block.id && hash(prev_block) == new_block.prev_hash;
}

fn is_valid_chain(chain: & Vec<Block>) -> bool{
	let mut prev = &chain[0];
	for block in chain[1..].iter() {
		if !is_valid_block(prev, block) {
			return false
		}
		prev = block;
	}
	return true
}

fn resolve<'a>(curr_chain: &'a Vec<Block>, new_chain: &'a Vec<Block>) -> &'a Vec<Block> {
	if curr_chain.len() < new_chain.len() && is_valid_chain(new_chain) {
		return new_chain;
	}
	return curr_chain;
}

fn main() {
	let mut blocks = vec![Block {
		id: 0,
		prev_hash: 0,
		data: "I'm awake!".to_string(),
	}];

	// this is necessary because rust 
	let mut next = next_block(blocks.last().unwrap(), "stop.".to_string());
	blocks.push(next);
	next = next_block(blocks.last().unwrap(), "hammertime.".to_string());
	blocks.push(next);

	println!("{:?}", blocks[0]);
	for bs in blocks[..blocks.len() - 1].iter().zip(blocks[1..].iter()) {
		let (b1, b2) = bs;
		println!("{:?} valid? {:?}", b2, is_valid_block(b1, b2));
	}
	println!("valid chain? {:?}", is_valid_chain(&blocks));
}
