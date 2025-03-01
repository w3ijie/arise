use rand::{Rng, distr::Distribution, rngs::ThreadRng, seq::IndexedRandom};

use crate::models::data::{DUNGEONS, DUNGEONS_WEIGHT_DISTRIBUTION, MONSTERS};

use super::{dungeon::Dungeon, monster::Monster};

pub struct Gate<'a> {
    pub dungeon: Dungeon<'a>,
    pub monsters: Vec<Monster<'a>>,
}

impl<'a> Gate<'a> {
    pub fn generate_random(rng: &mut ThreadRng) -> Gate<'a> {
        let dungeon = DUNGEONS[DUNGEONS_WEIGHT_DISTRIBUTION.sample(rng)].clone();

        let valid_monsters: Vec<Monster> = MONSTERS
            .iter()
            .filter(|m| m.dungeon_ranks.contains(&dungeon.rank))
            .cloned()
            .collect();

        if valid_monsters.len() == 0 {
            panic!("no valid monsters available for this dungeon")
        }

        let num_monsters = rng.random_range(1..=3);
        let monsters: Vec<Monster> = valid_monsters
            .choose_multiple(rng, num_monsters)
            .cloned()
            .collect();

        Gate { dungeon, monsters }
    }
}
