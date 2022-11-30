use serde::{Deserialize, Serialize};
use super::attribute::Attribute;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attributes {
    strength: Attribute,
    dexterity: Attribute,
    constitution: Attribute,
    intelligence: Attribute,
    wisdom: Attribute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EAttributes {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
}