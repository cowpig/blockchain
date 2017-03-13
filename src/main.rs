use std::hash::{Hash, Hasher};
use std::string::ToString;

#[derive(Debug)]
struct Block<'a> {
	id: u32,
	prev_hash: u64,
	this_hash: u64,
	data: &'a str,
}

impl<'a> Hash for Block<'a> {
	fn hash<H:Hasher>(&self, state:&mut H) {
		self.id.hash(state);
		self.this_hash.hash(state);
		self.prev_hash.hash(state);
		self.data.hash(state);
	}
}


fn main() {

}
