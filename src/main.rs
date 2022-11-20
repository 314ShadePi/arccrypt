use arccrypt::models::{block::Block, blocks::Blocks};

fn main() {
    let mut block = Block::new("0".to_string(), "Genesis Block".to_string());
    block.mine_block(5);
    let mut blocks = Blocks(vec![block]);
    let mut block = Block::new(blocks[blocks.len() - 1].hash.clone(), "LOOOL".to_string());
    block.mine_block(5);
    blocks.push(block);
    let mut block = Block::new(blocks[blocks.len() - 1].hash.clone(), "LOOOL".to_string());
    block.mine_block(5);
    blocks.push(block);
    let mut block = Block::new(blocks[blocks.len() - 1].hash.clone(), "LOOOL".to_string());
    block.mine_block(5);
    blocks.push(block);
    println!("{}", blocks);
}
