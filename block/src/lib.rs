use std::time::{SystemTime, UNIX_EPOCH};
use log::info;
use bincode::serialize;
use sha2::{Sha256, Digest};
pub type Result<T> = std::result::Result<T, failure::Error>;

const TARGET_DIFFICULTY: usize = 4;

#[allow(dead_code)]
struct Block {
    timestamp: u128,
    nonce: i32,
    transactions: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
}

impl Block {

    fn new_genesis_block() -> Result<Block> {
        Block::new_block(String::from(""), String::from(""), 0)
    }

    fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

        let mut block = Block {
            timestamp,
            nonce: 0,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height
        };

       block.run_pow()?;

        Ok(block)
    }

    fn run_pow(&mut self) -> Result<()> {
        info!("Block is mining");

        while !self.validate()? {
            self.nonce += 1
        }

        let mut hasher = Sha256::new();

        let serialized_block = self.serialize_block()?;

        hasher.update(&serialized_block);

        let hash_result = hasher.finalize();

        self.hash = format!("{:x}", hash_result);

        info!("Block has been mined");

        Ok(())
    }

    fn validate(&self) -> Result<bool> {
        // serializing the data
        let serialized_block= self.serialize_block()?;
       
        // instantiating Sha256
        let mut hasher = Sha256::new();
        
        // create MD from the serialized data 
        hasher.update(&serialized_block);
        let hash_result = hasher.finalize();

        let mut vec_one: Vec<u8>  = vec![];
        vec_one.resize(TARGET_DIFFICULTY, '0' as u8);
        println!("{:?}", vec_one);

        // compare result with set difficulty 
        Ok(hash_result[0..TARGET_DIFFICULTY] == vec_one)
    }

    // function for data serialization
    fn serialize_block(&self) -> Result<Vec<u8>> {
        let content = {
            self.prev_block_hash.clone();
            self.transactions.clone();
            self.timestamp;
            self.nonce;
            TARGET_DIFFICULTY  
        };

        let in_bytes = serialize(&content)?;
        Ok(in_bytes)
    } 
}


