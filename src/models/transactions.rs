use super::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transactions(pub Vec<Transaction>);

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
