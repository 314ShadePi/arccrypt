use arccrypt::models::{
    blockchain::Blockchain, transaction::Transaction,
};
use secp256k1::{KeyPair, Secp256k1, SecretKey};

fn main() {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&[
        0x2c, 0x14, 0x78, 0x6f, 0x24, 0x55, 0x32, 0xe1, 0x71, 0x7d, 0xaf, 0xd7, 0x07, 0x63, 0x9d,
        0x6e, 0xa0, 0x25, 0x22, 0x5c, 0x6d, 0xd7, 0x86, 0xb2, 0xb2, 0x01, 0xa9, 0x8d, 0x32, 0x53,
        0x74, 0xa6,
    ])
    .expect("32 bytes, within curve order");
    let key_pair = KeyPair::from_secret_key(&secp, &secret_key);
    let secret_key2 = SecretKey::from_slice(&[
        0x2e, 0x1e, 0x7e, 0x6e, 0x2e, 0x5e, 0xe2, 0xe1, 0x71, 0x7d, 0xaf, 0xd7, 0x07, 0x63, 0x9d,
        0x6e, 0xae, 0x2e, 0x2e, 0x5e, 0x6e, 0xde, 0x86, 0xb2, 0xb2, 0x01, 0xa9, 0x8d, 0x32, 0x53,
        0x7e, 0xae,
    ])
    .expect("32 bytes, within curve order");
    let key_pair2 = KeyPair::from_secret_key(&secp, &secret_key2);
    let mut coin = Blockchain::new();
    let mut tx1 = Transaction::new(Some(key_pair.public_key()), &key_pair2.public_key(), arccrypt::models::tx_payload::TXPayload::I64(10));
    tx1.sign_transaction(key_pair);
    coin.add_transaction(tx1);
    let mut tx1 = Transaction::new(Some(key_pair.public_key()), &key_pair2.public_key(), arccrypt::models::tx_payload::TXPayload::I64(20));
    tx1.sign_transaction(key_pair);
    coin.add_transaction(tx1);
    let mut tx1 = Transaction::new(Some(key_pair.public_key()), &key_pair2.public_key(), arccrypt::models::tx_payload::TXPayload::I64(30));
    tx1.sign_transaction(key_pair);
    coin.add_transaction(tx1);
    coin.mine_pending_transactions(key_pair.public_key());

    let mut tx2 = Transaction::new(Some(key_pair.public_key()), &key_pair2.public_key(), arccrypt::models::tx_payload::TXPayload::I64(30));
    tx2.sign_transaction(key_pair);
    coin.add_transaction(tx2);
    coin.mine_pending_transactions(key_pair.public_key());

    println!(
        "Balance of 1: {}",
        coin.get_balance_of_address(key_pair.public_key())
    );
    println!(
        "Balance of 2: {}",
        coin.get_balance_of_address(key_pair2.public_key())
    );
    coin.is_valid();
    println!("{}", serde_json::to_string_pretty::<Blockchain>(&coin).unwrap());
}
