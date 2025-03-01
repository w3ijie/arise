use super::rank::Rank;

#[derive(Debug, Clone)]
pub struct Dungeon<'a> {
    pub name: &'a str,
    pub spawn_rate: i32,
    pub rank: Rank,
}
impl<'a> Dungeon<'a> {
    pub fn new(name: &'a str, spawn_rate: i32, rank: Rank) -> Dungeon<'a> {
        Dungeon {
            name,
            spawn_rate,
            rank,
        }
    }
}
