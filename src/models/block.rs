use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: DateTime<Utc>,
    pub data: String,
    pub nonce: i128,
}

impl Block {
    pub fn new(previous_hash: String, data: String) -> Self {
        let mut intermediate = Self {
            hash: "".to_string(),
            previous_hash: previous_hash,
            timestamp: Utc::now(),
            data: data,
            nonce: 0,
        };

        intermediate.hash = intermediate.calculate_hash();
        intermediate
    }

    pub fn calculate_hash(&self) -> String {
        digest(format!(
            "{}{}{}{}",
            self.previous_hash, self.timestamp, self.data, self.nonce
        ))
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        println!("Mining block {} with difficulty {}", self.hash, difficulty);

        while self.hash[..difficulty] != vec!["0"; difficulty].join("") {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("Block mined to {}", self.hash);
    }
}
