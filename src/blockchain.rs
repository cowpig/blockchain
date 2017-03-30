use hash_utils::{hash_string};
use wordvote::{VoteChain};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Hash)]
pub struct Block {
	pub id: u64,
	pub prev_hash: String,
	pub data: VoteChain,
	// todo: add a timestamp
}

impl Block {
	pub fn next_block(&self, data: VoteChain) -> Block {
		return Block {
			id: self.id + 1,
			prev_hash: self.get_hash_string(),
			data: data,
		}
	}

	pub fn is_valid_next(&self, new_block: & Block) -> bool {
		return self.id + 1 == new_block.id && self.get_hash_string() == new_block.prev_hash;
	}

	pub fn get_hash_string(&self) -> String {
		let s = self.id.to_string() + &self.prev_hash + &self.data.get_hash_string();
		return hash_string(s);
	}
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Hash)]	 
pub struct Blockchain {
	pub blocks: Vec<Block>
}

impl Blockchain {
	pub fn extend(&mut self, data: VoteChain) {
		let last_block = self.blocks.last().unwrap().clone();
		let next_block = last_block.next_block(data);
		self.blocks.push(next_block);
	}

	pub fn is_valid(&self) -> bool{
		let mut prev = &self.blocks[0];
		for block in self.blocks[1..].iter() {
			if !prev.is_valid_next(block) {
				return false
			}
			prev = block;
		}
		return true
	}

	pub fn replaced_by(&self, other: & Blockchain) -> bool {
		return (self.blocks.len() < other.blocks.len()) && other.is_valid();
	}
}
