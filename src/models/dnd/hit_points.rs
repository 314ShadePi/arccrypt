use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitPoints {
    max: u32,
    current: i32,
    temp: i32,
}