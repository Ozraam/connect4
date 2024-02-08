use std::fmt::{Display, Formatter, Result};
use player::Player;

mod player;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellState {
    Empty,
    Red,
    Yellow,
}

#[derive(Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub type Board = [[CellState; 7]; 6];

pub struct Connect4 {
    board: Board,
    size: Size,
    turn: Player,
}



impl Connect4 {
    pub fn new() -> Connect4 {
        let board = [[CellState::Empty; 7]; 6];
        let size = Size {
            width: 7,
            height: 6,
        };
        Connect4 { board, size, turn: Player::Red}
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_board(&self) -> Board {
        self.board
    }

    pub fn play(&mut self, col: i32) -> bool {
        if col < 0 || col >= self.size.width as i32 {
            return false;
        }

        let mut i = 0;
        while i < self.size.height {
            if self.board[i as usize][col as usize] == CellState::Empty {
                self.board[i as usize][col as usize] = match self.turn {
                    Player::Red => CellState::Red,
                    Player::Yellow => CellState::Yellow,
                };

                self.turn = match self.turn {
                    Player::Red => Player::Yellow,
                    Player::Yellow => Player::Red,
                };

                return true;
            }
            i += 1;
        }
        false
    }

    /// Returns the winner if there is one
    pub fn is_someone_winning(&self) -> Option<Player> {
        for i in 0..self.size.height {
            for j in 0..self.size.width {
                if self.board[i as usize][j as usize] != CellState::Empty {
                    if self.check_win(i, j) {
                        return Some(match self.board[i as usize][j as usize] {
                            CellState::Red => Player::Red,
                            CellState::Yellow => Player::Yellow,
                            _ => unreachable!(),
                        });
                    }
                }
            }
        }
        None
    }

    fn check_win(&self, row: u32, col: u32) -> bool {
        let mut count = 0;
        let mut i = row;
        let mut j = col;

        // Check horizontal
        while j < self.size.width && self.board[i as usize][j as usize] == self.board[row as usize][col as usize] {
            count += 1;
            j += 1;
        }
        if count >= 4 {
            return true;
        }

        // Check vertical
        count = 0;
        i = row;
        j = col;
        while i < self.size.height && self.board[i as usize][j as usize] == self.board[row as usize][col as usize] {
            count += 1;
            i += 1;
        }
        if count >= 4 {
            return true;
        }

        // Check diagonal
        count = 0;
        i = row;
        j = col;
        while i < self.size.height && j < self.size.width && self.board[i as usize][j as usize] == self.board[row as usize][col as usize] {
            count += 1;
            i += 1;
            j += 1;
        }
        if count >= 4 {
            return true;
        }

        // Check anti-diagonal
        count = 0;
        i = row;
        j = col;
        while i < self.size.height && j > 0 && self.board[i as usize][j as usize - 1] == self.board[row as usize][col as usize] {
            count += 1;
            i += 1;
            j -= 1;
        }
        if count >= 4 {
            return true;
        }

        false
    }
}

impl Display for Connect4 {
    fn fmt(&self, f: &mut Formatter) -> Result {

        writeln!(f, "Connect 4")?;
        writeln!(f, "Player turn: {}", self.turn)?;
        writeln!(f, "  0 1 2 3 4 5 6")?;
        writeln!(f, "-----------------")?;
        for row in self.board.iter().rev() {
            write!(f, "| ")?;
            for cell in row.iter() {
                write!(f, "{}", match cell {
                    CellState::Empty => " ",
                    CellState::Red => "X",
                    CellState::Yellow => "O",
                })?;
                write!(f, " ")?;
            }
            write!(f, "|")?;
            writeln!(f)?;
        }
        write!(f, "-----------------")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let game = Connect4::new();
        assert_eq!(game.get_size().width, 7);
        assert_eq!(game.get_size().height, 6);
        assert_eq!(game.get_board(), [[CellState::Empty; 7]; 6]);
    }

    #[test]
    fn test_play_wrong() {
        let mut game = Connect4::new();
        assert_eq!(game.play(-1), false);
        assert_eq!(game.play(7), false);
    }

    #[test]
    fn test_play() {
        let mut game = Connect4::new();
        assert_eq!(game.play(0), true);

        let mut board = [[CellState::Empty; 7]; 6];
        board[0][0] = CellState::Red;
        assert_eq!(game.get_board(), board);
    }

    #[test]
    fn test_win_vertical() {
        let mut game = Connect4::new();
        game.play(0);
        game.play(1);
        game.play(0);
        game.play(1);
        game.play(0);
        game.play(1);
        game.play(0);
        assert_eq!(game.is_someone_winning().unwrap(), Player::Red);
    }

    #[test]
    fn test_win_horizontal() {
        let mut game = Connect4::new();
        game.play(0);
        game.play(0);
        game.play(1);
        game.play(1);
        game.play(2);
        game.play(2);
        game.play(3);
        assert_eq!(game.is_someone_winning().unwrap(), Player::Red);
    }

    #[test]
    fn test_win_diagonal() {
        let mut game = Connect4::new();
        game.play(0);
        game.play(1);
        game.play(1);
        game.play(2);
        game.play(2);
        game.play(3);
        game.play(2);
        game.play(3);
        game.play(3);
        game.play(5);
        game.play(3);
        assert_eq!(game.is_someone_winning().unwrap(), Player::Red);
    }
}
