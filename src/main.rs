mod block;

use block::{Block, Blockchain};


fn main() {
    let mut b = Blockchain::new();
        b.add_block("data".to_string());
}