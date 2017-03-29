extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512;


pub fn hash(s: &str) -> String {
	let mut hasher = Sha512::new();
	hasher.input_str(s);
	return hasher.result_str();
}



