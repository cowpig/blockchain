extern crate blockchain;

use blockchain::blockchain::{Block, is_valid_chain, replaces};

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
