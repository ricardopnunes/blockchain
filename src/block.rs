use std::time::SystemTime;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;
use serde::{Serialize, Deserialize};

// Define um novo tipo 'Result' que é um alias para o tipo Result<T, failure::Error> da biblioteca padrão de Rust.
pub type Result<T> = std::result::Result<T, failure::Error>; 

// Define uma constante chamada TARGET_HEXT com o valor 4 e do tipo usize. Essa constante representará o nivel de dificuldade da blockchain
const TARGET_HEXT: usize = 4; 

/*
A Block representa um bloco em uma blockchain e contém informações como:
    o carimbo de data/hora (timestamp), 
    as transações (transactions), 
    o hash do bloco anterior (prev_block_hash), 
    o hash do próprio bloco (hash), 
    a altura do bloco na cadeia (height) 
    e um valor nonce usado para mineração (nonce). 
*/


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block{
    timestamp: u128,           
    transactions: String,      
    perv_block_hash: String,   
    hash: String,              
    height: usize,             
    nonce: i32,                
}

impl Block	{

    pub fn get_prev_hash(&self) -> String{
        self.perv_block_hash.clone()
    }
    
    // Método público que retorna o hash do bloco.
    pub fn get_hash(&self) -> String { 
        self.hash.clone()               // Retorna o hash do bloco como uma cópia da string.
    }

    // Método estático para criar o bloco gênese, "Priemiro BLoco".
    pub fn new_genesis_block() -> Block{ 
        // Cria um novo bloco com dados específicos para o bloco gênese.
        Block::new_block(String::from("Primeiro Bloco"), String::new(), 0).unwrap() 
    }

    // Método estático para criar um novo bloco.
    pub fn new_block(data: String, perv_block_hash: String, height: usize) -> Result<Block>{ 
        // Obtém o carimbo de data/hora atual em milissegundos.
        let timestamp: u128 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis(); 

        // Inicializa um novo bloco com os parâmetros fornecidos.
        let mut block = Block{ 
            timestamp: timestamp,
            transactions: data,
            perv_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        
        // Executa o processo de prova de trabalho para encontrar um hash válido para o bloco.
        block.run_proof_if_work()?; 
        
        // Retorna o bloco criado com sucesso.
        Ok(block) 
    }

    fn run_proof_if_work(&mut self) -> Result<()>{
        info!("Minerando o bloco"); // Registra uma mensagem informativa de que o bloco está sendo minerado.
    
        // Faço o Loop enquanto o bloco não for válido de acordo com a função 'validate'.
        while !self.validate()?{
            self.nonce += 1; // Incrementa o valor do nonce para tentar encontrar um hash válido.
        }

        // Prepara os dados para hashing.
        let data = self.prepare_hash_data()?;  
        
        // Inicializa um novo objeto hasher usando o algoritmo SHA-256.
        let mut hasher = Sha256::new();         
        
        // Adiciona os dados preparados para hashing.
        hasher.input(&data[..]);                        
        
        // Calcula o hash final e o armazena no campo 'hash' do bloco.
        self.hash = hasher.result_str();                
        
        // Retorna Ok(()) para indicar que a operação foi bem-sucedida.
        Ok(())                                         
    }
    

    fn prepare_hash_data(&self) -> Result<Vec<u8>>{
        // Cria uma tupla com os dados necessários para calcular o hash.
        let content = (
            self.perv_block_hash.clone(),   // Hash do bloco anterior.
            self.transactions.clone(),      // Transações do bloco.
            self.timestamp,                 // Carimbo de data/hora do bloco.
            TARGET_HEXT,                    // Valor do alvo (target).
            self.nonce                      // Valor do nonce.
        );
    
        // Serializa a tupla em bytes usando o formato binário.
        let bytes = bincode::serialize(&content)?;
    
        // Retorna os bytes serializados.
        Ok(bytes)
    }
    
    fn validate(&self) -> Result<bool>{
        // Prepara os dados para hashing.
        let data = self.prepare_hash_data()?; 
        
        // Inicializa um novo objeto hasher usando o algoritmo SHA-256.
        let mut hasher = Sha256::new();       
        
        // Adiciona os dados preparados para hashing.
        hasher.input(&data[..]);              
        
        // Inicializa um novo vetor vazio de bytes.
        let mut vec1: Vec<u8> = vec![];       
        
        // Define o vetor com 'TARGET_HEXT' zeros.
        vec1.resize(TARGET_HEXT, '0' as u8);  
    
        // Compara os primeiros 'TARGET_HEXT' caracteres do hash calculado com 'vec1'.
        // Retorna true se forem iguais, caso contrário, retorna false.
        Ok(&hasher.result_str()[0..TARGET_HEXT] == String::from_utf8(vec1)?)
    }
    
}
