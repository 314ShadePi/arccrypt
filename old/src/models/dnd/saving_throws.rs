use serde::{Deserialize, Serialize};

use super::saving_throw::SavingThrow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingThrows {
    strength: SavingThrow,
    dexterity: SavingThrow,
    constitution: SavingThrow,
    intelligence: SavingThrow,
    wisdom: SavingThrow,
    charisma: SavingThrow,
}