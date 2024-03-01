use std::time::SystemTime;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;


pub type Result<T> = std::result::Result<T, failure::Error>;
const TARGET_HEXT: usize = 4;

pub struct Block{
    timestamp: u128,
    transactions: String,
    perv_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

pub struct Blockchain {
    blocks: Vec<Block>
}

impl Block	{
    
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn new_genesis_block() -> Block{
        Block::new_block(String::from("Primeiro Bloco"), String::new(), 0).unwrap()
    }

    pub fn new_block(data: String, perv_block_hash: String, height: usize) -> Result<Block>{
        let timestamp: u128 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
        
        let mut block = Block{
            timestamp: timestamp,
            transactions: data,
            perv_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        
        block.run_proof_if_work()?;
        Ok(block)
    }

    fn run_proof_if_work(&mut self) -> Result<()>{
        info!("Minerando o bloco");
        while !self.validate()?{
            self.nonce += 1;
        }

        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>>{
        let content = (
            self.perv_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce
        );
        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }


    fn validate(&self) -> Result<bool>{
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = vec![];
        vec1.resize(TARGET_HEXT, '0' as u8);
        println!("{:?}", vec1);
        Ok(&hasher.result_str()[0..TARGET_HEXT] == String::from_utf8(vec1)?)
    }

}


impl Blockchain{
    pub fn new() -> Blockchain{
        Blockchain{
            blocks: vec![Block::new_genesis_block()]
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()>{
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash(), TARGET_HEXT)?;
        self.blocks.push(new_block);
        Ok(())
    }
}