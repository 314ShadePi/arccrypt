use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathSaves {
    successes: [bool; 3],
    failiures: [bool; 3],
}