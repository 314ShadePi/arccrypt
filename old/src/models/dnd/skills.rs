use serde::{Deserialize, Serialize};

use super::skill::Skill;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    acrobatics: Skill,
    animal_handling: Skill,
    arcana: Skill,
    athletics: Skill,
    deception: Skill,
    history: Skill,
    insight: Skill,
    intimidation: Skill,
    investigation: Skill,
    medicine: Skill,
    nature: Skill,
    perception: Skill,
    performance: Skill,
    persuasion: Skill,
    religion: Skill,
    sleight_of_hands: Skill,
    stealth: Skill,
    survival: Skill,
}
