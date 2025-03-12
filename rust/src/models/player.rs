use chrono::{DateTime, Local};
use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use super::{
    ability::Ability,
    data::ABILITY_SHADOW_MINIONS,
    monster::Monster,
    shadow::{Shadow, ShadowRank},
};

const DEFAULT_HEALTH: i32 = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub health: i32,
    //pub journey_started: DateTime<Local>,
    pub shadows: Vec<Shadow>,

    #[serde(skip)]
    pub rng: ThreadRng,
}

impl Player {
    pub fn new(name: Option<&str>) -> Player {
        let name = match name {
            Some(v) => v,
            None => "Sung Jin Woo",
        };

        Player {
            name: name.to_string(),
            health: DEFAULT_HEALTH,
            //journey_started: Local::now(),
            shadows: vec![],
            rng: rand::rng(),
        }
    }

    pub fn attack(&mut self, monster: &mut Monster, ability: &Ability) -> i32 {
        let damage = match ability.name {
            ABILITY_SHADOW_MINIONS => {
                let mut acc = 0;
                let _ = self.shadows.iter().map(|m| acc += m.power * 5);
                acc
            }
            _ => ability.damage,
        };

        monster.health -= damage;
        damage
    }

    pub fn attempt_arise(&mut self, monster: &mut Monster) -> Option<Shadow> {
        if !monster.is_defeated() {
            //|| monster.arise_attempts <= 0 {
            return None;
        }

        //monster.arise_attempts -= 1;

        if self.rng.random_range(1..=2) == 1 {
            let rank = ShadowRank::from_monster(monster);

            let shadow = Shadow::new(&monster.name, rank, monster.power);
            self.shadows.push(shadow.clone());
            return Some(shadow);
        }

        None
    }

    pub fn is_defeated(&self) -> bool {
        self.health <= 0
    }
}
