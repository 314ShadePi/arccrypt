use super::transactions::Transactions;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: DateTime<Utc>,
    pub transactions: Transactions,
    pub nonce: i128,
}

impl Block {
    pub fn new(previous_hash: String, data: Transactions) -> Self {
        let mut intermediate = Self {
            hash: "".to_string(),
            previous_hash,
            timestamp: Utc::now(),
            transactions: data,
            nonce: 0,
        };

        intermediate.hash = intermediate.calculate_hash();
        intermediate
    }

    pub fn calculate_hash(&self) -> String {
        digest(format!(
            "{}{}{}{}",
            self.previous_hash, self.timestamp, self.transactions, self.nonce
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

    pub fn is_valid(&self) -> bool {
        for tx in self.transactions.clone().0 {
            if !tx.is_valid() {
                return false;
            } else {
                continue;
            }
        }

        true
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty::<Block>(self).unwrap()
        )
    }
}
