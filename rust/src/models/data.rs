use once_cell::sync::Lazy;
use rand::distr::weighted::WeightedIndex;

use super::{ability::Ability, dungeon::Dungeon, monster::Monster, rank::Rank};

pub const ABILITY_SHADOW_MINIONS: &'static str = "Shadow Minions Attack";

pub const ABILITIES: Lazy<Vec<Ability<'static>>> = Lazy::new(|| {
    vec![
        Ability::new("Dagger Slash", 15, 30),
        Ability::new("Stealth Attack", 25, 25),
        Ability::new("Quicksilver", 30, 20),
        Ability::new(ABILITY_SHADOW_MINIONS, 0, 25),
    ]
});

pub const MONSTERS: Lazy<Vec<Monster<'static>>> = Lazy::new(|| {
    vec![
        Monster::new("Goblin", 2, vec![Rank::E]),
        Monster::new("Hobgoblin", 3, vec![Rank::E, Rank::D]),
        Monster::new("Orc", 4, vec![Rank::D, Rank::C]),
        Monster::new("Ogre", 5, vec![Rank::D, Rank::C]),
        Monster::new("Lizardman", 6, vec![Rank::C, Rank::B]),
        Monster::new("Minotaur", 7, vec![Rank::B]),
        Monster::new("Giant", 8, vec![Rank::B, Rank::A]),
        Monster::new("Demon", 9, vec![Rank::A, Rank::S]),
        Monster::new("Dragon", 10, vec![Rank::S]),
    ]
});

pub const DUNGEONS: Lazy<Vec<Dungeon<'static>>> = Lazy::new(|| {
    vec![
        Dungeon::new("E-Rank Dungeon", 30, Rank::E),
        Dungeon::new("D-Rank Dungeon", 25, Rank::D),
        Dungeon::new("C-Rank Dungeon", 20, Rank::C),
        Dungeon::new("B-Rank Dungeon", 15, Rank::B),
        Dungeon::new("A-Rank Dungeon", 7, Rank::A),
        Dungeon::new("S-Rank Dungeon", 3, Rank::S),
    ]
});

pub const DUNGEONS_WEIGHT_DISTRIBUTION: Lazy<WeightedIndex<i32>> = Lazy::new(|| {
    let weights: Vec<i32> = DUNGEONS.iter().map(|d| d.spawn_rate).collect();
    WeightedIndex::new(&weights).unwrap()
});
