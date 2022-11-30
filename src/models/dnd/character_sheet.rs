use super::{personality::Personality, skills::Skills, attributes::Attributes};
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
    class: String,
    level: i64,
    background: String,
    race: String,
    alignment: String,
    personality: Personality,
    skills: Skills,
    attributes: Attributes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSheetUpdate {
    id: Uuid,
    owner: PublicKey,
    timestamp: DateTime<Utc>,
    name: Option<String>,
}

impl CharacterSheet {
    pub fn new(
        owner: &PublicKey,
        name: String,
        player_name: String,
        class: String,
        level: i64,
        background: String,
        race: String,
        alignment: String,
        personality: Personality,
        skills: Skills,
        attributes: Attributes,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner: *owner,
            timestamp: Utc::now(),
            name,
            player_name,
            class,
            level,
            background,
            race,
            alignment,
            personality,
            skills,
            attributes,
        }
    }
}
