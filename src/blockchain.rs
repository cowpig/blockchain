use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Block {
	pub id: u64,
	pub prev_hash: u64,
	pub data: String,
}

pub type Blockchain = Vec<Block>;

pub fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    return s.finish()
}

impl Hash for Block {
	fn hash<H:Hasher>(&self, state:&mut H) {
		self.id.hash(state);
		self.prev_hash.hash(state);
		self.data.hash(state);
	}
}

impl Block {
	pub fn next_block(&self, data: String) -> Block {
		return Block {
			id: self.id + 1,
			prev_hash: get_hash(self),
			data: data,
		}
	}

	pub fn is_valid_next(&self, new_block: & Block) -> bool {
		return self.id + 1 == new_block.id && get_hash(self) == new_block.prev_hash;
	}
}

pub fn is_valid_chain(chain: & Blockchain) -> bool{
	let mut prev = &chain[0];
	for block in chain[1..].iter() {
		if !prev.is_valid_next(block) {
			return false
		}
		prev = block;
	}
	return true
}

pub fn replaces(curr_chain: & Blockchain, new_chain: & Blockchain) -> bool {
	return (curr_chain.len() < new_chain.len()) && is_valid_chain(new_chain)
}
