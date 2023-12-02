use std::time::{SystemTime, UNIX_EPOCH};
pub type Result<T> = std::result::Result<T, failure::Error>;

struct Block {
    timestamp: u128,
    nonce: i32,
    transactions: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
}

impl Block {
    fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

        let block = Block {
            timestamp: timestamp,
            nonce: 0,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height
        };

        Ok(block)
    }
}