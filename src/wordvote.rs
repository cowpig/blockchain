extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512;

use hash_utils::{hash_string, hash_bytes};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Vote {
	pub pub_id: String,
	pub last_hash: String,
	pub nonce: String,
}

impl Vote {
	pub fn concat_string(&self) -> String {
		return (self.pub_id.clone() + &self.last_hash + &self.nonce).clone();
	}

	pub fn is_valid_nonce(&self, n_bytes: usize, max_remainder: u8) -> bool {
		let bytes = hash_bytes(self.concat_string());
		for byte in bytes[..n_bytes].iter() {
			if *byte != 0 {
				return false;
			}
		}
		return bytes[n_bytes] < max_remainder;
	}

	pub fn get_hash_string(&self) -> String{
		return hash_string(self.concat_string());
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct VoteChain {
	pub word: String,
	pub votes: Vec<Vote>
}

impl VoteChain {
	pub fn is_valid(&self, n_bytes: usize, max_remainder: u8) -> bool {
		if self.votes.len() > 0 {
			return true
		}
		if self.votes[0].last_hash != hash_string(self.word.clone()) {
			return false
		}
		if !self.votes[0].is_valid_nonce(n_bytes, max_remainder) {
			return false
		}
		let mut prev = & self.votes[0];
		for vote in self.votes[1..].iter() {
			if prev.get_hash_string() != vote.last_hash {
				return false
			}
			if !vote.is_valid_nonce(n_bytes, max_remainder) {
				return false
			}
			prev = vote;
		}
		return true
	}

	pub fn get_hash_string(&self) -> String {
		let mut hasher = Sha512::new();
		for vote in self.votes.iter() {
			hasher.input_str(&vote.concat_string());
		}
		return hasher.result_str();
	}

	pub fn replaced_by(&self, other: &VoteChain, n_bytes: usize, max_remainder: u8) -> bool {
		return (self.votes.len() < other.votes.len()) && other.is_valid(n_bytes, max_remainder);
	}
}
