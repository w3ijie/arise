use super::rank::Rank;

#[derive(Debug, Clone)]
pub struct Monster<'a> {
    pub name: &'a str,
    pub health: i32,
    pub power: i32,
    pub dungeon_ranks: Vec<Rank>,
    //pub arise_attempts: i32,
}

impl<'a> Monster<'a> {
    pub fn new(name: &'a str, power: i32, allowed_ranks: Vec<Rank>) -> Monster<'a> {
        Monster {
            name,
            health: power * 10,
            power,
            dungeon_ranks: allowed_ranks,
            //arise_attempts: 0,
        }
    }

    pub fn is_defeated(&self) -> bool {
        self.health <= 0
    }
}
