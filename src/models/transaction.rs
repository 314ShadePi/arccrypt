use secp256k1::{ecdsa::Signature, KeyPair, Message, PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};
use sha256::digest;
use super::tx_payload::TXPayload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from_address: Option<PublicKey>,
    pub to_address: PublicKey,
    pub payload: TXPayload,
    signature: Option<Signature>,
}

impl Transaction {
    pub fn new(from_address: Option<PublicKey>, to_address: &PublicKey, amount: TXPayload) -> Self {
        Self {
            from_address,
            to_address: *to_address,
            payload: amount,
            signature: None,
        }
    }

    pub fn calculate_hash(&self) -> String {
        digest(format!(
            "{}{:#?}{}",
            self.to_address,
            self.payload,
            self.from_address.unwrap()
        ))
    }

    pub fn sign_transaction(&mut self, signing_key: KeyPair) -> bool {
        let secp = Secp256k1::new();
        if signing_key.public_key() != self.from_address.unwrap() {
            false
        } else {
            let message = Message::from_hashed_data::<bitcoin_hashes::sha256::Hash>(
                self.calculate_hash().as_bytes(),
            );
            let signature = secp.sign_ecdsa(&message, &signing_key.secret_key());
            self.signature = Some(signature);
            true
        }
    }

    pub fn is_valid(&self) -> bool {
        println!("Validating tx...");
        let secp = Secp256k1::new();
        if self.from_address.is_none() {
            true
        } else if self.signature.is_none() || self.signature.unwrap().to_string().is_empty() {
            false
        } else {
            secp.verify_ecdsa(
                &Message::from_hashed_data::<bitcoin_hashes::sha256::Hash>(
                    self.calculate_hash().as_bytes(),
                ),
                &self.signature.unwrap(),
                &self.from_address.unwrap(),
            ).is_ok()
        }
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty::<Transaction>(self).unwrap()
        )
    }
}
