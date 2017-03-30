extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512;

pub fn hash_string(s: String) -> String {
	let mut hasher = Sha512::new();
	hasher.input_str(s.as_ref());
	return hasher.result_str();
}

pub fn hash_bytes<'a>(s: String) -> [u8; 64] {
	let mut hasher = Sha512::new();
	hasher.input_str(s.as_ref());
	let output: &mut [u8; 64] = &mut [0; 64];
	hasher.result(output);
	return *output;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
	pub_id: String,
	last_hash: String,
	nonce: String,
}

impl Vote {
	fn concat_string(&self) -> String {
		return (self.pub_id.clone() + &self.last_hash + &self.nonce).clone();
	}

	fn is_valid_nonce(&self, n_bytes: usize, max_remainder: u8) -> bool {
		let mut bytes = hash_bytes(self.concat_string());
		for byte in bytes[..n_bytes].iter() {
			if *byte != 0 {
				return false;
			}
		}
		return bytes[n_bytes] < max_remainder;
	}

	fn get_hash_string(&self) -> String{
		return hash_string(self.concat_string());
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordVote {
	word: String,
	votes: Vec<Vote>
}

impl WordVote {
	fn is_valid(&self, n_bytes: usize, max_remainder: u8) -> bool {
		if self.votes.len() > 0 {
			return true
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
}
