use secp256k1::rand::rngs::OsRng;
use secp256k1::{KeyPair, Secp256k1};

fn main() {
    let secp = Secp256k1::new();
    let (secret_key, _) = secp.generate_keypair(&mut OsRng);
    let key_pair = KeyPair::from_secret_key(&secp, &secret_key);
    println!("{:#?}", key_pair.secret_key().secret_bytes());
    println!("{:#?}", key_pair.public_key());
}
