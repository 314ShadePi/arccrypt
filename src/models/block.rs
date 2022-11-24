use super::transactions::Transactions;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    hash: String,
    previous_hash: String,
    timestamp: DateTime<Utc>,
    nonce: i128,
    transactions: Transactions,
}

impl Block {
    /// Creates a new block with the given previous_hash and data.
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

    /// Calculates the hash of the block.
    pub fn calculate_hash(&self) -> String {
        digest(format!(
            "{}{}{}{}",
            self.previous_hash, self.timestamp, self.transactions, self.nonce
        ))
    }

    /// Mines the block by changing the nonce value.
    pub fn mine_block(&mut self, difficulty: usize) {
        println!("Mining block {} with difficulty {}", self.hash, difficulty);

        while self.hash[..difficulty] != vec!["0"; difficulty].join("") {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("Block mined to {}", self.hash);
    }

    /// Checks if the block has valid transactions
    pub fn is_valid(&self) -> bool {
        #[cfg(debug_assertions)]
        println!("Validating block...");
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

impl Block {
    /// Returns the block hash.
    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    /// Returns the previous blocks hash.
    pub fn previous_hash(&self) -> String {
        self.previous_hash.clone()
    }

    /// Returns the timestamp of the creation of the block.
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    /// Returns the nonce of the block.
    pub fn nonce(&self) -> i128 {
        self.nonce
    }

    /// Returns the transactions of the block.
    pub fn transactions(&self) -> Transactions {
        self.transactions.clone()
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
