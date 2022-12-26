use serde::{Deserialize, Serialize};

use crate::prelude::Payload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TXPayload {
    String(String),
    I64(i64),
    Payload(Box<dyn Payload>),
}
