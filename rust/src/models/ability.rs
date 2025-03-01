#[derive(Debug, Clone)]
pub struct Ability<'a> {
    pub name: &'a str,
    pub damage: i32,

    // Flagged as unused
    pub spawn_rate: i32,
}

impl<'a> Ability<'a> {
    pub fn new(name: &'a str, damage: i32, spawn_rate: i32) -> Ability<'a> {
        Ability {
            name,
            damage,
            spawn_rate,
        }
    }
}
