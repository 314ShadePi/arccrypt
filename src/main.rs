use arccrypt::models::block::Block;
use sha256::digest;

fn main() {
    let block = Block::new(digest("input"), "Hello World".to_string());
    println!("{:#?}", block);
    let mut block = block;
    block.mine_block(5);
    let block = block;
    println!("{:#?}", block);
}
