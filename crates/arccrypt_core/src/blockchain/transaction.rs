use super::tx_payload::TXPayload;
use secp256k1::{ecdsa::Signature, KeyPair, Message, PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    from_address: PublicKey,
    to_address: PublicKey,
    signature: Signature,
    payload: TXPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transactions(pub Vec<Transaction>);

#[derive(Debug, Clone)]
pub struct TransactionBuilder {
    from_address: Option<PublicKey>,
    to_address: Option<PublicKey>,
    signature: Option<Signature>,
    payload: Option<TXPayload>,
}

impl Transaction {
    pub fn builder() -> TransactionBuilder {
        TransactionBuilder {
            from_address: None,
            to_address: None,
            signature: None,
            payload: None,
        }
    }

    pub fn calculate_hash(&self) -> String {
        digest(format!(
            "{}{}{}",
            self.to_address,
            serde_json::to_string_pretty(&self.payload).unwrap(),
            self.from_address
        ))
    }

    pub fn is_valid(&self) -> bool {
        #[cfg(debug_assertions)]
        println!("Validating tx...");
        let secp = Secp256k1::new();
        if self.signature.to_string().is_empty() {
            false
        } else {
            secp.verify_ecdsa(
                &Message::from_hashed_data::<bitcoin_hashes::sha256::Hash>(
                    self.calculate_hash().as_bytes(),
                ),
                &self.signature,
                &self.from_address,
            )
            .is_ok()
        }
    }
}

impl Transaction {
    pub fn from_address(&self) -> PublicKey {
        self.from_address
    }

    pub fn to_address(&self) -> PublicKey {
        self.to_address
    }

    pub fn payload(&self) -> TXPayload {
        self.payload.clone()
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

impl std::fmt::Display for Transactions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.iter().fold(Ok(()), |result, block| {
            result.and_then(|_| writeln!(f, "{}", block))
        })
    }
}

impl std::ops::Deref for Transactions {
    type Target = Vec<Transaction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Transactions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TransactionBuilder {
    pub fn from(&mut self, from: &PublicKey) -> &mut Self {
        let mut new = self;
        new.from_address = Some(from.to_owned());
        new
    }

    pub fn to(&mut self, to: &PublicKey) -> &mut Self {
        let mut new = self;
        new.to_address = Some(to.to_owned());
        new
    }

    pub fn payload(&mut self, payload: TXPayload) -> &mut Self {
        let mut new = self;
        new.payload = Some(payload);
        new
    }

    fn calculate_hash(&self) -> String {
        digest(format!(
            "{}{}{}",
            self.to_address.unwrap(),
            serde_json::to_string_pretty(&self.payload.as_ref().unwrap()).unwrap(),
            self.from_address.unwrap()
        ))
    }

    pub fn build(&mut self, signing_key: KeyPair) -> Result<Transaction, String> {
        if let None = self.from_address {
            return Err("missing from".to_string());
        }

        if let None = self.to_address {
            return Err("missing to".to_string());
        }

        if let None = self.payload {
            return Err("missing payload".to_string());
        }

        let secp = Secp256k1::new();
        if signing_key.public_key() != self.from_address.unwrap() {
            return Err("Signing key not from_address".to_string());
        } else {
            let message = Message::from_hashed_data::<bitcoin_hashes::sha256::Hash>(
                self.calculate_hash().as_bytes(),
            );
            let signature = secp.sign_ecdsa(&message, &signing_key.secret_key());
            self.signature = Some(signature);
        }

        Ok(Transaction {
            from_address: Clone::clone(
                self.from_address
                    .as_ref()
                    .ok_or("missing from".to_string())?,
            ),
            to_address: Clone::clone(self.to_address.as_ref().ok_or("missing from".to_string())?),
            signature: Clone::clone(self.signature.as_ref().ok_or("missing from".to_string())?),
            payload: Clone::clone(self.payload.as_ref().ok_or("missing from".to_string())?),
        })
    }
}
