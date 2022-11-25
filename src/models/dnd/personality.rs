use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    traits: String,
    ideals: String,
    bonds: String,
    flaws: String,
}