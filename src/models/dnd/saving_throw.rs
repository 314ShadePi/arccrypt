use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingThrow {
    value: u8,
    activated: bool,
}