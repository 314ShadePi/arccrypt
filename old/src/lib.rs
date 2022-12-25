pub mod models;
pub mod traits;

pub mod prelude {
    pub use crate::traits::payload::Payload;
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::{blockchain::Blockchain, transaction::Transaction, tx_payload::TXPayload};
    use secp256k1::{KeyPair, Secp256k1, SecretKey};

    fn create_sample_chain() -> Blockchain {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[
            0x2c, 0x14, 0x78, 0x6f, 0x24, 0x55, 0x32, 0xe1, 0x71, 0x7d, 0xaf, 0xd7, 0x07, 0x63,
            0x9d, 0x6e, 0xa0, 0x25, 0x22, 0x5c, 0x6d, 0xd7, 0x86, 0xb2, 0xb2, 0x01, 0xa9, 0x8d,
            0x32, 0x53, 0x74, 0xa6,
        ])
        .expect("32 bytes, within curve order");
        let key_pair = KeyPair::from_secret_key(&secp, &secret_key);
        let secret_key2 = SecretKey::from_slice(&[
            0x2e, 0x1e, 0x7e, 0x6e, 0x2e, 0x5e, 0xe2, 0xe1, 0x71, 0x7d, 0xaf, 0xd7, 0x07, 0x63,
            0x9d, 0x6e, 0xae, 0x2e, 0x2e, 0x5e, 0x6e, 0xde, 0x86, 0xb2, 0xb2, 0x01, 0xa9, 0x8d,
            0x32, 0x53, 0x7e, 0xae,
        ])
        .expect("32 bytes, within curve order");
        let key_pair2 = KeyPair::from_secret_key(&secp, &secret_key2);
        let mut coin = Blockchain::new(2, 100);
        let mut tx1 = Transaction::new(
            &key_pair.public_key(),
            &key_pair2.public_key(),
            TXPayload::I64(10),
        );
        tx1.sign_transaction(key_pair);
        coin.add_transaction(tx1);
        let mut tx1 = Transaction::new(
            &key_pair.public_key(),
            &key_pair2.public_key(),
            TXPayload::I64(20),
        );
        tx1.sign_transaction(key_pair);
        coin.add_transaction(tx1);
        let mut tx1 = Transaction::new(
            &key_pair.public_key(),
            &key_pair2.public_key(),
            TXPayload::I64(30),
        );
        tx1.sign_transaction(key_pair);
        coin.add_transaction(tx1);
        coin.mine_pending_transactions(key_pair.public_key());

        for _ in 0..10 {
            for _ in 0..10 {
                let mut tx1 = Transaction::new(
                    &key_pair.public_key(),
                    &key_pair2.public_key(),
                    TXPayload::I64(10),
                );
                tx1.sign_transaction(key_pair);
                coin.add_transaction(tx1);
            }
            coin.mine_pending_transactions(key_pair.public_key());
        }

        coin.clone()
    }

    #[test]
    fn is_chain_valid() {
        let coin = create_sample_chain();
        assert!(coin.is_valid());
    }
}
