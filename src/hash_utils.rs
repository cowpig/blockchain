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
