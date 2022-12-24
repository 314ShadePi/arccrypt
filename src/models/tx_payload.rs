use serde::{Deserialize, Serialize};

use crate::prelude::Payload;

use super::dnd::character_sheet::CharacterSheet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TXPayload {
    String(String),
    I64(i64),
    CharacterSheet(CharacterSheet),
    Custom(Box<dyn Payload>),
}
