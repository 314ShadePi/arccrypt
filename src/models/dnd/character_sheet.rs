use super::personality::Personality;
use chrono::prelude::*;
use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSheet {
    id: Uuid,
    owner: PublicKey,
    timestamp: DateTime<Utc>,
    name: String,
    player_name: String,
    class_lvl: String,
    background: String,
    race: String,
    alignment: String,
    exp: i128,
    str: i64,
    dex: i64,
    con: i64,
    int: i64,
    wis: i64,
    cha: i64,
    personality: Personality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSheetUpdate {
    id: Uuid,
    owner: PublicKey,
    timestamp: DateTime<Utc>,
    name: Option<String>,
}

impl CharacterSheet {
    pub fn new(owner: &PublicKey, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner: *owner,
            timestamp: Utc::now(),
            name,
            player_name: todo!(),
            class_lvl: todo!(),
            background: todo!(),
            race: todo!(),
            alignment: todo!(),
            exp: todo!(),
            str: todo!(),
            dex: todo!(),
            con: todo!(),
            int: todo!(),
            wis: todo!(),
            cha: todo!(),
            personality: todo!(),
        }
    }
}
