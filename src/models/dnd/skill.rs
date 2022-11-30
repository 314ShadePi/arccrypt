use serde::{Deserialize, Serialize};

use super::attributes::EAttributes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    active: bool,
    amount: i32,
    modifier: EAttributes,
}