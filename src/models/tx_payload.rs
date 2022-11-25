use serde::{Deserialize, Serialize};

use super::dnd::character_sheet::{CharacterSheet, CharacterSheetUpdate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TXPayload {
    String(String),
    I64(i64),
    CharacterSheet(CharacterSheet),
    CharacterSheetUpdate(CharacterSheetUpdate)
}