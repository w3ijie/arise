use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rank {
    S,
    A,
    B,
    C,
    D,
    E,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::S => write!(f, "S-Rank"),
            Rank::A => write!(f, "A-Rank"),
            Rank::B => write!(f, "B-Rank"),
            Rank::C => write!(f, "C-Rank"),
            Rank::D => write!(f, "D-Rank"),
            Rank::E => write!(f, "E-Rank"),
        }
    }
}

