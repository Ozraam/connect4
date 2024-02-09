use std::fmt::Display;

use crate::CellState;

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

impl Player {
    pub fn to_cell_state(&self) -> CellState {
        match self {
            Player::Red => CellState::Red,
            Player::Yellow => CellState::Yellow,
        }
    }
}