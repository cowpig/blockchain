extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512;

pub fn hash_string(s: String) -> String {
	let mut hasher = Sha512::new();
	hasher.input_str(s.to_ref());
	return hasher.result_str();
}

pub fn hash_bytes(s: String) -> &mut [u8] {
	let mut hasher = Sha512::new();
	hasher.input_str(s.to_ref());
	return hasher.result();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
	pub_id: String,
	last_hash: String,
	nonce: String,
}

pub impl Vote {
	fn to_string(&self) {
		return self.pub_id + &self.last_hash + &self.nonce;
	}

	fn is_valid(&self, n_bytes: usize, max_remainder: usize) {
		let mut bytes = hash_bytes(self.to_string());
		for byte in bytes[..n_bytes].iter() {
			if byte != 0 {
				return false;
			}
		}
		return bytes[n_bytes] < max_remainder;
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordVote {
	word: String,
	votes: Vec<Vote>
}

pub impl WordVote {

}