use btclib::crypto::PrivateKey;
use btclib::sha256::Hash;
use btclib::types::{Block, BlockHeader, Transaction, TransactionOutput};
use btclib::util::{MerkleRoot, Saveable};
use chrono::Utc;
use std::env;
use std::process::exit;
use uuid::Uuid;

fn main() {
    let path = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        eprintln!("Usage: block_gen <output_block_file_path>");
        exit(1);
    };
    let private_key = PrivateKey::new_key();
    let transactions = vec![Transaction::new(
        vec![],
        vec![TransactionOutput {
            unique_id: Uuid::new_v4(),
            value: btclib::INITIAL_REWARD * 10u64.pow(8),
            pubkey: private_key.public_key(),
        }],
    )];
    let merkel_root = MerkleRoot::calculate(&transactions);
    let block = Block::new(
        BlockHeader {
            timestamp: Utc::now(),
            merkle_root: merkel_root,
            target: btclib::MIN_TARGET,
            nonce: 0,
            prev_block_hash: Hash::zero(),
        },
        transactions,
    );
    block.save_to_file(path).expect("Failed to save block");
}
