mod block;
mod errors;
mod blockchain;

fn main() {
    let mut b = blockchain::Blockchain::new().unwrap();
        b.add_block("data".to_string());
}