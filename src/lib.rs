use std::fmt::{self, Display, Formatter};
use player::Player;

mod player;
mod evaluator;

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
    moves: Vec<u32>,
}

impl Connect4 {
    pub fn new() -> Connect4 {
        let board = [[CellState::Empty; 7]; 6];
        let size = Size {
            width: 7,
            height: 6,
        };
        Connect4 { board, size, turn: Player::Red, moves: Vec::new()}
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_board(&self) -> Board {
        self.board
    }

    pub fn get_turn(&self) -> &Player {
        &self.turn
    }

    pub fn play(&mut self, col: u32) -> bool {
        if col >= self.size.width {
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
                self.moves.push(col);
                return true;
            }
            i += 1;
        }
        false
    }

    pub fn undo(&mut self) -> Result<(), String> {
        if self.moves.is_empty() {
            return Err("Move list empty".into());
        }

        let col = self.moves.pop().unwrap();
        let mut i = (self.size.height - 1) as i32;

        while i >= 0 {
            if self.board[i as usize][col as usize] != CellState::Empty {
                self.board[i as usize][col as usize] = CellState::Empty;
                self.turn = match self.turn {
                    Player::Red => Player::Yellow,
                    Player::Yellow => Player::Red,
                };
                return Ok(());
            }
            i -= 1;
        }
        Err("Column empty".into())
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
        let mut i = row as i32;
        let mut j = col as i32;

        // Check vertical
        while i >= 0 {
            if self.board[i as usize][col as usize] == self.board[row as usize][col as usize] {
                count += 1;
            } else {
                break;
            }
            i -= 1;
        }
        i = row as i32 + 1;
        while i < self.size.height as i32 {
            if self.board[i as usize][col as usize] == self.board[row as usize][col as usize] {
                count += 1;
            } else {
                break;
            }
            i += 1;
        }
        if count >= 4 {
            return true;
        }

        // Check horizontal
        count = 0;
        while j >= 0 {
            if self.board[row as usize][j as usize] == self.board[row as usize][col as usize] {
                count += 1;
            } else {
                break;
            }
            j -= 1;
        }
        j = col as i32 + 1;
        while j < self.size.width as i32 {
            if self.board[row as usize][j as usize] == self.board[row as usize][col as usize] {
                count += 1;
            } else {
                break;
            }
            j += 1;
        }
        if count >= 4 {
            return true;
        }

        // Check diagonal
        count = 0;
        i = row as i32;
        j = col as i32;
        while i >= 0 && j >= 0 {
            if self.board[i as usize][j as usize] == self.board[row as usize][col as usize] {
                count += 1;
            } else {
                break;
            }
            i -= 1;
            j -= 1;
        }
        i = row as i32 + 1;
        j = col as i32 + 1;
        while i < self.size.height as i32 && j < self.size.width as i32 {
            if self.board[i as usize][j as usize] == self.board[row as usize][col as usize] {
                count += 1;
            } else {
                break;
            }
            i += 1;
            j += 1;
        }
        if count >= 4 {
            return true;
        }

        // Check anti-diagonal
        count = 0;
        i = row as i32;
        j = col as i32;
        while i >= 0 && j < self.size.width as i32 {
            if self.board[i as usize][j as usize] == self.board[row as usize][col as usize] {
                count += 1;
            } else {
                break;
            }
            i -= 1;
            j += 1;
        }
        i = row as i32 + 1;
        j = col as i32 - 1;
        while i < self.size.height as i32 && j >= 0 {
            if self.board[i as usize][j as usize] == self.board[row as usize][col as usize] {
                count += 1;
            } else {
                break;
            }
            i += 1;
            j -= 1;
        }
        if count >= 4 {
            return true;
        }

        false
    }

    pub fn print_board(&self) {
        println!("{}", self);
    }


    pub fn is_draw(&self) -> bool {
        for i in 0..self.size.width {
            for j in 0..self.size.height {
                if self.board[j as usize][i as usize] == CellState::Empty {
                    return false;
                }
            }
        }
        true
    }

    pub fn play_minimax(&mut self, depth: i32) {
        let bot_move = evaluator::find_best_move(self, depth);
        self.play(bot_move);
    }

    pub fn get_cell(&self, i: u32, j: u32) -> Option<&Player>  {
        if i >= self.size.height || j >= self.size.width {
            return None;
        }
        match self.board[i as usize][j as usize] {
            CellState::Red => Some(&Player::Red),
            CellState::Yellow => Some(&Player::Yellow),
            _ => None,
        }
    }

}

impl Display for Connect4 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

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
        assert_eq!(game.play(8), false);
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

    #[test]
    fn test_undo() -> Result<(), String> {
        let mut game = Connect4::new();
        game.play(0);
        game.undo()?;
        assert_eq!(game.get_board(), [[CellState::Empty; 7]; 6]);
        Ok(())
    }
}
