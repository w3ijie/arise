use std::{cmp::min, fmt};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::monster::Monster;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadowRank {
    Soldier,
    Elite,
    Knight,
    EliteKnight,
    Commander,
    Marshal,
}

impl fmt::Display for ShadowRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShadowRank::Soldier => write!(f, "Soldier"),
            ShadowRank::Elite => write!(f, "Elite"),
            ShadowRank::Knight => write!(f, "Knight"),
            ShadowRank::EliteKnight => write!(f, "Elite Knight"),
            ShadowRank::Commander => write!(f, "Commander"),
            ShadowRank::Marshal => write!(f, "Marshal"),
        }
    }
}

pub const SHADOW_RANKS: [ShadowRank; 6] = [
    ShadowRank::Soldier,
    ShadowRank::Elite,
    ShadowRank::Knight,
    ShadowRank::EliteKnight,
    ShadowRank::Commander,
    ShadowRank::Marshal,
];

impl ShadowRank {
    pub fn from_monster(monster: &Monster) -> ShadowRank {
        let index = min(SHADOW_RANKS.len() - 1, (monster.power / 2) as usize);
        SHADOW_RANKS[index].clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shadow {
    pub name: String,
    pub rank: ShadowRank,
    pub power: i32,
    pub arised_at: DateTime<Local>,
}

impl Shadow {
    pub fn new(name: &str, rank: ShadowRank, power: i32) -> Shadow {
        Shadow {
            name: name.to_string(),
            rank,
            power,
            arised_at: chrono::Local::now(),
        }
    }
}
