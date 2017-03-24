extern crate blockchain;

use blockchain::blockchain::{Block, next_block, is_valid_block, is_valid_chain, resolve};

#[test]
fn test_blockchain() {
	let mut blocks = vec![Block {
		id: 0,
		prev_hash: 0,
		data: "I'm awake!".to_string(),
	}];

	// this is necessary because rust 
	let mut next = next_block(blocks.last().unwrap(), "stop.".to_string());
	blocks.push(next);
	next = next_block(blocks.last().unwrap(), "hammertime.".to_string());
	blocks.push(next);

	// println!("{:?}", blocks[0]);
	for bs in blocks[..blocks.len() - 1].iter().zip(blocks[1..].iter()) {
		let (b1, b2) = bs;
		assert!(is_valid_block(b1, b2));
		// println!("{:?} valid? {:?}", b2, is_valid_block(b1, b2));
	}
	assert!(is_valid_chain(&blocks));
	// println!("valid chain? {:?}", is_valid_chain(&blocks));

	assert_eq!(blocks[0], blocks[0].clone());

	assert_eq!(blocks, *resolve(& blocks, & blocks[..2].to_vec()));
}
