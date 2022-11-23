use super::{block::Block, blocks::Blocks, transaction::Transaction, transactions::Transactions};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{KeyPair, PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub difficulty: usize,
    pub chain: Blocks,
    pub pending_transactions: Transactions,
    pub mining_reward: i64,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            difficulty: 5,
            chain: Blocks(vec![Self::create_genesis_block()]),
            pending_transactions: Transactions(vec![]),
            mining_reward: 100,
        }
    }

    fn create_genesis_block() -> Block {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut OsRng);
        let key_pair1 = KeyPair::from_secret_key(&secp, &secret_key);
        let (secret_key, _) = secp.generate_keypair(&mut OsRng);
        let key_pair2 = KeyPair::from_secret_key(&secp, &secret_key);
        Block::new(
            "0".to_string(),
            Transactions(vec![Transaction::new(
                Some(key_pair1.public_key()),
                &key_pair2.public_key(),
                10,
            )]),
        )
    }

    pub fn get_latest_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    pub fn mine_pending_transactions(&mut self, mining_reward_address: PublicKey) {
        let reward_tx = Transaction::new(None, &mining_reward_address, self.mining_reward);
        self.pending_transactions.push(reward_tx);
        let mut block = Block::new(
            self.get_latest_block().hash.clone(),
            self.pending_transactions.clone(),
        );
        block.mine_block(self.difficulty);
        self.chain.push(block);
        self.pending_transactions = Transactions(vec![]);
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        if tx.from_address.is_none() || tx.to_address.to_string().len() == 0 {
            return;
        }

        if !tx.is_valid() {
            return;
        }

        self.pending_transactions.push(tx);
    }

    pub fn get_balance_of_address(&self, address: PublicKey) -> i64 {
        let mut balance: i64 = 0;
        for block in self.chain.clone().0 {
            for tx in block.transactions.clone().0 {
                if tx.from_address.unwrap() == address {
                    balance -= tx.amount;
                }

                if tx.to_address == address {
                    balance += tx.amount;
                }
            }
        }
        balance
    }

    pub fn is_valid(&self) -> bool {
        let mut i = 1;
        while i < self.chain.len() {
            let current_block = self.chain[i].clone();
            let previous_block = self.chain[i - 1].clone();

            if !current_block.is_valid() {
                return false;
            }

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }

            i = 1 + 1;
        }

        return true;
    }
}
