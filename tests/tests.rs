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
	    "word": "what",
	    "votes": [
			{
			  "pub_id": "max",
			  "last_hash": "813db7fa66134df5295d98c5abbf90ff7206d68f3372a25138ee9c2bbb4c96d22f978ffd3da550f8dc38a15e106bec5266f91bc8447241b79e4ae0ce9fb8ff88",
			  "nonce": ""
			},
			{
			  "pub_id": "max",
			  "last_hash": "711fcabd230670030e8c4eacca7add668f5de5611d286c2c8f39380ca1e1b34db8a305832d2283722d151c3a575c3972cf9301fae02a8825aa63ea5ee1fde0ad",
			  "nonce": "488906987642"
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

