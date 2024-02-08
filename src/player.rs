use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Player {
    Red,
    Yellow,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::Red => write!(f, "Red X"),
            Player::Yellow => write!(f, "Yellow O"),
        }
    }
}