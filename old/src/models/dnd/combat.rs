use serde::{Deserialize, Serialize};

use super::{death_saves::DeathSaves, hit_dice::HitDice, hit_points::HitPoints};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combat {
    armor_class: i8,
    initiative: i32,
    speed: u16,
    hit_points: HitPoints,
    death_saves: DeathSaves,
    hit_dice: HitDice,
}