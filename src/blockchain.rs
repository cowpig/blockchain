use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Block {
	pub id: u64,
	pub prev_hash: u64,
	pub data: String,
}

pub type Blockchain = Vec<Block>;

impl Hash for Block {
	fn hash<H:Hasher>(&self, state:&mut H) {
		self.id.hash(state);
		self.prev_hash.hash(state);
		self.data.hash(state);
	}
}

pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn next_block(prev_block: & Block, data: String) -> Block {
	Block {
		id: prev_block.id + 1,
		prev_hash: hash(prev_block),
		data: data,
	}
}

pub fn is_valid_block(prev_block: & Block, new_block: & Block) -> bool {
	(prev_block.id + 1 == new_block.id) && (hash(prev_block) == new_block.prev_hash)
}

pub fn is_valid_chain(chain: & Blockchain) -> bool{
	let mut prev = &chain[0];
	for block in chain[1..].iter() {
		if !is_valid_block(prev, block) {
			return false
		}
		prev = block;
	}
	return true
}
