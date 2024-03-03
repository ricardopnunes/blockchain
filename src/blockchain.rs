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
    pub fn new() -> Result<Blockchain> {
        // Abre ou cria um banco de dados no diretório 'data/blocks'
        let db = sled::open("data/blocks")?;
    
        // Verifica se existe um valor associado à chave "LAST" no banco de dados
        match db.get("LAST")? {
            Some(hash) => {
                // Se existir, converte o valor para uma String
                let last_hash = String::from_utf8(hash.to_vec())?;
                // Retorna uma instância de Blockchain com o último hash conhecido
                Ok(Blockchain {
                    current_hash: last_hash,
                    db,
                })
            }
            None => {
                // Se não existir, cria um bloco genesis
                let block = Block::new_genesis_block();
                // Insere o bloco no banco de dados
                db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                // Atualiza o último hash conhecido
                db.insert("LAST", block.get_hash().as_bytes())?;
                // Cria uma nova instância de Blockchain com o bloco genesis
                let bc = Blockchain {
                    current_hash: block.get_hash(),
                    db,
                };
                // Limpa a memória
                bc.db.flush()?;
                // Retorna a instância de Blockchain
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

        // Insere o hash do novo bloco e o bloco serializado no banco de dados
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;

        // Atualiza o último hash conhecido no banco de dados
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;

        // Atualiza o hash atual da blockchain para o hash do novo bloco
        self.current_hash = new_block.get_hash();

        // Retorna Ok(()) para indicar que a operação foi bem-sucedida.
        Ok(())                       
    }

    // Definição da função iter que retorna um BlockchainIter
    pub fn iter(&self) -> BlockchainIter {
        // Cria e retorna um novo BlockchainIter com o hash atual e uma referência à blockchain atual
        BlockchainIter {
            current_hash: self.current_hash.clone(), // Clona o hash atual para o iterador
            bc: &self, // Passa uma referência à própria blockchain para o iterador
        }
    }

}


// Implementação do método next para o iterador BlockchainIter
impl<'a> Iterator for BlockchainIter<'a> {
    // O tipo de item que o iterador irá retornar
    type Item = Block;

    // Definição do método next, que retorna o próximo bloco na blockchain
    fn next(&mut self) -> Option<Self::Item> {
        // Verifica se é possível obter o bloco serializado a partir do hash atual
        if let Ok(encoded_block) = self.bc.db.get(&self.current_hash) {
            // Se for possível, tenta desserializar o bloco
            // Utiliza um 'match' para lidar com o resultado de 'encoded_block', que é uma operação de busca no banco de dados da blockchain.
            return match encoded_block {
                // Se 'encoded_block' for 'Some', significa que foi encontrado um bloco serializado no banco de dados.
                Some(b) => {
                    // Tenta desserializar o bloco serializado ('b') passando a referencia 'bincode::deserialize::<Block>(&b)'.
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        // Se a desserialização for bem-sucedida, atualiza o 'current_hash' do iterador para o hash do bloco anterior.
                        self.current_hash = block.get_prev_hash();
                        // Retorna 'Some' com o bloco desserializado. Isso significa que o próximo item retornado pelo iterador será este bloco.
                        Some(block)
                    } else {
                        // Se a desserialização falhar, o bloco não é do tipo 'Block' esperado. Neste caso, retorna 'None', indicando que o próximo item do iterador não pode ser fornecido.
                        None
                    }
                }
                // Se 'encoded_block' for 'None', significa que não foi encontrado nenhum bloco correspondente ao hash atual no banco de dados. 
                // Retorna 'None', indicando que não há mais itens a serem fornecidos pelo iterador.
                None => None,
            };

        }
        None // Retorna None se não for possível obter o bloco serializado
    }
}


use super::*;


//Função para testes.
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