use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TXPayload {
    String(String),
    I64(i64),
}