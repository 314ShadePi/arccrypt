use super::tx_payload::TXPayload;
use super::{block::Block, blocks::Blocks, transaction::Transaction, transactions::Transactions};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{KeyPair, PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    difficulty: usize,
    mining_reward: i64,
    pending_transactions: Transactions,
    chain: Blocks,
}

impl Blockchain {
    pub fn new(difficulty: usize, mining_reward: i64) -> Self {
        Self {
            difficulty,
            pending_transactions: Transactions(vec![]),
            mining_reward,
            chain: Blocks(vec![Self::create_genesis_block()]),
        }
    }

    fn create_genesis_block() -> Block {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut OsRng);
        let key_pair1 = KeyPair::from_secret_key(&secp, &secret_key);
        let (secret_key, _) = secp.generate_keypair(&mut OsRng);
        let key_pair2 = KeyPair::from_secret_key(&secp, &secret_key);
        let mut tx = Transaction::new(
                &key_pair1.public_key(),
                &key_pair2.public_key(),
                TXPayload::I64(10),
        );
        tx.sign_transaction(key_pair1);
        let mut intermediate = Block::new(
            "0".to_string(),
            Transactions(vec![tx]),
        );
        intermediate.mine_block(2);
        intermediate
    }

    pub fn get_latest_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    pub fn mine_pending_transactions(&mut self, mining_reward_address: PublicKey) {
        if !self.is_valid() {
            return;
        }

        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut OsRng);
        let key_pair = KeyPair::from_secret_key(&secp, &secret_key);

        let mut reward_tx = Transaction::new(
            &key_pair.public_key(),
            &mining_reward_address,
            TXPayload::I64(self.mining_reward),
        );
        reward_tx.sign_transaction(key_pair);

        self.pending_transactions.push(reward_tx);
        let mut block = Block::new(
            self.get_latest_block().hash(),
            self.pending_transactions.clone(),
        );
        block.mine_block(self.difficulty);
        self.chain.push(block);
        self.pending_transactions = Transactions(vec![]);
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        if !self.is_valid() {
            return;
        }

        if tx.from_address().to_string().is_empty() || tx.to_address().to_string().is_empty() {
            return;
        }

        if !tx.is_valid() {
            return;
        }

        self.pending_transactions.push(tx);
    }

    pub fn get_balance_of_address(&self, address: PublicKey) -> Result<i64, String> {
        if !self.is_valid() {
            return Err("Blockchain invalid".to_string());
        }

        let mut balance: i64 = 0;
        for block in self.chain.clone().0 {
            for tx in block.transactions().clone().0 {
                match tx.payload() {
                    TXPayload::String(_) => todo!(),
                    TXPayload::I64(pl) => {
                        if tx.from_address() == address {
                            balance -= pl;
                        }

                        if tx.to_address() == address {
                            balance += pl;
                        }
                    }
                    TXPayload::CharacterSheet(_) => todo!(),
                    TXPayload::CharacterSheetUpdate(_) => todo!(),
                }
            }
        }
        Ok(balance)
    }

    pub fn is_valid(&self) -> bool {
        let mut i = 0;
        #[cfg(debug_assertions)]
        println!("Validating chain...");
        while i < self.chain.len() {
            let current_block = self.chain[i].clone();

            if !current_block.is_valid() {
                return false;
            }
            if i > 0 {
                let previous_block = self.chain[i - 1].clone();

                if current_block.hash() != current_block.calculate_hash() {
                    return false;
                }

                if current_block.previous_hash() != previous_block.hash() {
                    return false;
                }
            }

            i += 1;
        }

        let mut i = 0;
        while i < self.pending_transactions.len() {
            if !self.pending_transactions[i].is_valid() {
                return false;
            }

            i += 1;
        }

        true
    }
}

impl Blockchain {
    pub fn difficulty(&self) -> usize {
        self.difficulty
    }

    pub fn mining_reward(&self) -> i64 {
        self.mining_reward
    }

    pub fn pending_transactions(&self) -> Transactions {
        self.pending_transactions.clone()
    }

    pub fn chain(&self) -> Blocks {
        self.chain.clone()
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new(5, 100)
    }
}
