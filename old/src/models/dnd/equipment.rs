use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    value: Vec<String>,
    cp: i32,
    sp: i32,
    ep: i32,
    gp: i32,
    pp: i32,
}