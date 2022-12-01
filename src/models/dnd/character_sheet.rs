use super::{
    attributes::Attributes, combat::Combat, equipment::Equipment, personality::Personality,
    saving_throws::SavingThrows, skills::Skills,
};
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
    experience_points: i128,
    age: u32,
    height: u8,
    weight: u8,
    eyes: String,
    skin: String,
    hair: String,
    inspiration: u8,
    proficiency_bonus: u8,
    passive_wisdom_perception: i8,
    other_proficiencies_and_languages: Vec<String>,
    features_and_traits: Vec<String>,
    character_backstory: String,
    treasure: Vec<String>,
    attributes: Attributes,
    saving_throws: SavingThrows,
    skills: Skills,
    combat: Combat,
    personality: Personality,
    equipment: Equipment,
}

impl CharacterSheet {
    pub fn new(owner: &PublicKey) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner: *owner,
            timestamp: Utc::now(),
            name: todo!(),
            player_name: todo!(),
            class: todo!(),
            level: todo!(),
            background: todo!(),
            race: todo!(),
            alignment: todo!(),
            experience_points: todo!(),
            age: todo!(),
            height: todo!(),
            weight: todo!(),
            eyes: todo!(),
            skin: todo!(),
            hair: todo!(),
            inspiration: todo!(),
            proficiency_bonus: todo!(),
            passive_wisdom_perception: todo!(),
            other_proficiencies_and_languages: todo!(),
            features_and_traits: todo!(),
            character_backstory: todo!(),
            treasure: todo!(),
            attributes: todo!(),
            saving_throws: todo!(),
            skills: todo!(),
            combat: todo!(),
            personality: todo!(),
            equipment: todo!(),
        }
    }
}
