extern crate blockchain;

use blockchain::blockchain::{Block, is_valid_chain, replaces};
use blockchain::wordvote::{hash_string, hash_bytes};

#[test]
fn test_blockchain() {
	let mut blocks = vec![Block {
		id: 0,
		prev_hash: 0,
		data: "I'm awake!".to_string(),
	}];

	// this is necessary because rust
	let next = blocks[0].next_block("stop.".to_string());
	let next2 = next.next_block("hammertime.".to_string());
	blocks.push(next);
	blocks.push(next2);

	// println!("{:?}", blocks[0]);
	for bs in blocks[..blocks.len() - 1].iter().zip(blocks[1..].iter()) {
		let (b1, b2) = bs;
		assert!(b1.is_valid_next(b2));
		// println!("{:?} valid? {:?}", b2, is_valid_block(b1, b2));
	}
	assert!(is_valid_chain(&blocks));
	// println!("valid chain? {:?}", is_valid_chain(&blocks));

	assert_eq!(blocks[0], blocks[0].clone());

	assert_eq!(false, replaces(& blocks, & blocks[..2].to_vec()));
}


#[test]
fn test_wordvote() {
	let hex_str = hash_string("hello world".to_string());
	assert_eq!(hex_str,
		   concat!("309ecc489c12d6eb4cc40f50c902f2b4",
				   "d0ed77ee511a7c7a9bcd3ca86d4cd86f",
				   "989dd35bc5ff499670da34255b45b0cf",
				   "d830e81f605dcf7dc5542e93ae9cd76f"));

	let bytes = hash_bytes("hello world".to_string());
	let mut string = "".to_string();
	for byte in bytes.iter() {
		let next_str = format!("{:02x}", byte);
		println!("{}", next_str);
		string += &next_str;
	}
	assert_eq!(hex_str, string);
}

