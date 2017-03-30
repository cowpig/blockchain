extern crate blockchain;
extern crate serde_json;

use blockchain::blockchain::{Block, Blockchain};
use blockchain::wordvote::{Vote, VoteChain};
use blockchain::hash_utils::{hash_string, hash_bytes};

#[test]
fn test_blockchain() {
	let mut blocks = Blockchain {
		blocks: vec![Block {
			id: 0,
			prev_hash: "".to_string(),
			data: VoteChain {
				word: "".to_string(),
				votes: vec![],
			},
		}]
	};

	let votechain = serde_json::from_str(r#"
	{
		"word": "hello world",
		"votes": [
			{
				"pub_id": "bananaman",
				"last_hash": "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f",
				"nonce": "0"
			}
		]
	}
	"#).unwrap();

	blocks.extend(votechain);

	assert!(blocks.is_valid());

	assert_eq!(blocks.blocks[0], blocks.blocks[0].clone());

	assert_eq!(false, blocks.replaced_by(&blocks));
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

