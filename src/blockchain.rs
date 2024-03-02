use crate::block::*;
use sled;
/*
    A Blockchain é uma estrutura que contém uma lista de blocos (blocks), 
    representada como um vetor de Block. Ela é usada para armazenar e gerenciar a cadeia de blocos.
*/


#[derive(Debug, Clone)]
pub struct Blockchain {
    current_hash: String,
    db: sled::Db,
}

pub struct BlockchainIter<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}


impl Blockchain{
    // Método estático para criar uma nova instância de Blockchain, iniciando com o bloco gênese.
    pub fn new() -> Result<Blockchain>{
        let db = sled::open("data/blocks")?;
        match db.get("LAST")? {
            Some(hash) =>{
                let last_hash = String::from_utf8(hash.to_vec())?;
                Ok(Blockchain {
                    current_hash: last_hash,
                    db,
                })
            } None=> {
                let block = Block::new_genesis_block();
                db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                db.insert("LAST", block.get_hash().as_bytes())?;
                let bc = Blockchain {
                    current_hash: block.get_hash(),
                    db,
                };
                bc.db.flush()?;
                Ok(bc)
            }
        }
    }

    // Método para adicionar um novo bloco à cadeia de blocos.
    pub fn add_block(&mut self, data: String) -> Result<()>{
        // Obtém o último bloco na cadeia.
        let last_hash = self.db.get("LAST")?.unwrap(); 
        
        // Cria um novo bloco com os dados fornecidos, o hash do último bloco como hash anterior e o valor do alvo.
        let new_block = Block::new_block(data, String::from_utf8(last_hash.to_vec())?, 4)?;
        
        // Adiciona o novo bloco à cadeia de blocos.
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        // Retorna Ok(()) para indicar que a operação foi bem-sucedida.
        Ok(())                       
    }

    pub fn iter(&self) -> BlockchainIter{
        BlockchainIter { 
            current_hash: self.current_hash.clone(), 
            bc: &self 
        }
    }
}


impl <'a> Iterator for BlockchainIter<'a> {
    type Item =  Block;

    fn next(&mut self) -> Option<Self::Item>{
        if let Ok(encoded_block) = self.bc.db.get(&self.current_hash){
            return match encoded_block {
                Some(b) =>{
                    if let Ok(block) = bincode::deserialize::<Block>(&b){
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    }else{
                        None
                    }
                }
                None => None
            };
        }
        None
    }
}

use super::*;

#[test]
fn test_add_block(){
    let mut b = Blockchain::new().unwrap();
    b.add_block("data 1".to_string());
    b.add_block("data 2".to_string());
    b.add_block("data 3".to_string());

    for item in b.iter(){
        println!("Item {:?}", item)
    }
}