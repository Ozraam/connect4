use std::fmt::{self, Display, Formatter};
use player::Player;
use rand::Rng;
use rustbenchmarktimer::timer::BenchmarkTimer;

mod player;
// mod evaluator;
// pub mod server;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellState {
    Empty,
    Red,
    Yellow,
}

impl From<CellState> for Option<Player> {
    fn from(cell: CellState) -> Self {
        match cell {
            CellState::Empty => None,
            CellState::Red => Some(Player::Red),
            CellState::Yellow => Some(Player::Yellow),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
    pub length: u32,
}

#[derive(Debug, PartialEq)]
pub struct Board {
    pub yellow: u64,
    pub red: u64,
    pub size: Size,
}

impl Board {
    pub fn new() -> Board {
        Board {
            yellow: 0,
            red: 0,
            size: Size {
                width: 7,
                height: 6,
                length: 42,
            },
        }
    }

    pub fn combined(&self) -> u64 {
        self.yellow | self.red
    }

    pub fn get_cell(&self, i: u32, j: u32) -> CellState {
        if i >= self.size.height || j >= self.size.width {
            return CellState::Empty;
        }
        let pos = 1 << (i * self.size.width + j);
        if self.red & pos != 0 {
            CellState::Red
        } else if self.yellow & pos != 0 {
            CellState::Yellow
        } else {
            CellState::Empty
        }
    }
}

pub struct Connect4 {
    board: Board,
    size: Size,
    turn: Player,
    moves: Vec<u32>,
    winning_masks: Vec<u64>,
}

impl Connect4 {
    pub fn new() -> Connect4 {
        let board = Board::new();
        let size = Size {
            width: 7,
            height: 6,
            length: 42,
        };

        // Initialize the winning sequence
        let mut winning_masks = Vec::new();
        // Horizontal
        let line1 = 0b1111;
        for i in 0..(size.height as u64) {
            for j in 0..(size.width as u64 - 3) {
                winning_masks.push(line1 << (i * size.width as u64 + j));
            }
        }
        // Vertical
        let col1 = 1 << 0 | 1 << size.width | 1 << size.width * 2 | 1 << size.width * 3;
        for i in 0..(size.width as u64) {
            for j in 0..(size.height as u64 - 3) {
                winning_masks.push(col1 << (i + j * size.width as u64));
            }
        }
        // Diagonal \
        let diag1 = 1 << 3 | 1 << size.width + 2 | 1 << size.width * 2 + 1 | 1 << size.width * 3;
        for i in 0..(size.width as u64 - 3) {
            for j in 0..(size.height as u64 - 3) {
                winning_masks.push(diag1 << (i + j * size.width as u64));
            }
        }
        // Diagonal /
        let diag2 = 1 << 0 | 1 << size.width + 1 | 1 << size.width * 2 + 2 | 1 << size.width * 3 + 3;
        for i in 0..(size.width as u64 - 3) {
            for j in 0..(size.height as u64 - 3) {
                winning_masks.push(diag2 << (i + j * size.width as u64));
            }
        }

        Connect4 { board, size, turn: Player::Red, moves: Vec::new(), winning_masks }
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_turn(&self) -> &Player {
        &self.turn
    }

    pub fn play(&mut self, col: u32) -> bool {
        if col >= self.size.width {
            return false;
        }

        let mut pos = 1 << col;

        let com = self.board.combined();

        while com & pos != 0 {
            
            
            pos <<= self.size.width;
        }

        if pos > 1 << (self.size.length - 1) {
            return false;
        }

        if self.turn == Player::Red {
            self.board.red |= pos;
        } else {
            self.board.yellow |= pos;
        }

        self.moves.push(col);

        self.turn = if self.turn == Player::Red {
            Player::Yellow
        } else {
            Player::Red
        };

        true
    }

    pub fn undo(&mut self) -> Result<(), String> {
        if self.moves.is_empty() {
            return Err("Move list empty".into());
        }

        let col = self.moves.pop().unwrap();
        let mut mask : u64 = (u64::MAX - 1).rotate_left(col);
        let com = self.board.combined();
        while com & mask != com {
            mask = mask.rotate_left(self.size.width as u32);
        }
        mask = mask.rotate_right(self.size.width as u32);


        if self.turn == Player::Red {
            self.board.yellow &= mask;
        } else {
            self.board.red &= mask;
        }

        self.turn = if self.turn == Player::Red {
            Player::Yellow
        } else {
            Player::Red
        };

        Ok(())
    }


    /// Returns the winner if there is one
    pub fn is_someone_winning(&self) -> Option<Player> {
        for mask in self.winning_masks.iter() {
            if self.board.red & mask == *mask {
                return Some(Player::Red);
            } else if self.board.yellow & mask == *mask {
                return Some(Player::Yellow);
            }

        }
        None
    }

    pub fn print_board(&self) {
        println!("{}", self);
    }


    pub fn is_draw(&self) -> bool {
        // Check if the board is full and no one is winning
        self.board.combined() == (1 << self.size.length) - 1 && self.is_someone_winning().is_none()
    }

    pub fn play_minimax(&mut self, depth: i32) -> u32 {
        // let mut bench = BenchmarkTimer::new();
        // bench.start("botplay");
        // let bot_move = evaluator::find_best_move(self, depth, &mut Some(&mut bench));
        // self.play(bot_move);
        // bench.stop("botplay");
        // bench.print();
        // bot_move
        0
    }

    pub fn get_cell(&self, i: u32, j: u32) -> Option<Player>  {
        if i >= self.size.height || j >= self.size.width {
            return None;
        }
        return self.board.get_cell(i, j).into();
    }


    fn get_hash(&self) -> u64 {
        // Use Zobrist hashing for better performance
        thread_local! {
            static ZOBRIST_TABLE: [[[u64; 3]; 7]; 6] = {
                let mut rng = rand::thread_rng();
                let mut table = [[[0; 3]; 7]; 6];
                // Fill the table with random values
                for i in 0..6 {
                    for j in 0..7 {
                        for k in 0..3 {
                            table[i][j][k] = rng.gen();
                        }
                    }
                }
                table
            };
        }

        let mut hash: u64 = 0;
        ZOBRIST_TABLE.with(|table| {
            // XOR the hash with the appropriate random number for each position
            for i in 0..self.size.height {
                for j in 0..self.size.width {
                    let cell_value = self.board.get_cell(i, j) as usize;
                    if cell_value > 0 {
                        hash ^= table[i as usize][j as usize][cell_value - 1];
                    }
                }
            }
        });

        hash
    }
}

impl Display for Connect4 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        writeln!(f, "Connect 4")?;
        writeln!(f, "Player turn: {}", self.turn)?;
        writeln!(f, "  0 1 2 3 4 5 6")?;
        writeln!(f, "-----------------")?;
        for i in 0..self.size.height {
            write!(f, "|")?;
            for j in 0..self.size.width {
                match self.get_cell(i, j) {
                    Some(Player::Red) => write!(f, "R|")?,
                    Some(Player::Yellow) => write!(f, "Y|")?,
                    None => write!(f, " |")?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "-----------------")?;
        Ok(())
    }
}

fn print_mask(mask: u64) {
    for i in 0..6 {
        for j in 0..7 {
            let pos = 1 << (i * 7 + j);
            if mask & pos != 0 {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
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
        assert_eq!(*game.get_board(), Board::new());
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

        assert_eq!(game.get_board().red, 1 << 0);
        assert_eq!(game.get_turn(), &Player::Yellow);
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
    fn test_win_diagonal_right() {
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
        game.print_board();
        assert_eq!(game.is_someone_winning().unwrap(), Player::Red);
    }

    #[test]
    fn test_win_diagonal_left() {
        let mut game = Connect4::new();
        game.play(3);
        game.play(2);
        game.play(2);
        game.play(1);
        game.play(1);
        game.play(0);
        game.play(1);
        game.play(0);
        game.play(0);
        game.play(2);
        game.play(0);
        game.print_board();
        assert_eq!(game.is_someone_winning().unwrap(), Player::Red);
    }

    #[test]
    fn test_no_win() {
        let mut game = Connect4::new();
        game.play(0);
        game.play(1);
        game.play(2);
        game.play(3);
        assert_eq!(game.is_someone_winning(), None);
    }

    #[test]
    fn test_undo() -> Result<(), String> {
        let mut game = Connect4::new();
        game.play(0);
        game.undo()?;
        assert_eq!(game.get_board(), &Board::new());
        Ok(())
    }

    #[test]
    fn test_draw() {
        let mut game = Connect4::new();
        for y in 0..3 {
            for i in 0..7 {
                game.play((i + 1 * y%2) % 7);
            }
        }
        assert_eq!(game.is_draw(), false);
        assert_eq!(game.is_someone_winning(), None);

        for y in 0..3 {
            for i in 0..7 {
                game.play((i + 1 * y%2 ) % 7);
            }
        }
        assert_eq!(game.is_draw(), true);
    }

    #[test]
    fn test_illegal_move() {
        let mut game = Connect4::new();
        game.play(0);
        game.play(0);
        game.play(0);
        game.play(0);
        game.play(0);
        game.play(0);
        assert_eq!(game.play(0), false);
        assert_eq!(game.play(1), true);
        assert_eq!(game.play(1), true);
        assert_eq!(game.play(1), true);
        assert_eq!(game.play(1), true);
        assert_eq!(game.play(1), true);
        assert_eq!(game.play(1), true);
        assert_eq!(game.play(1), false);
        assert_eq!(game.play(2), true);
        assert_eq!(game.play(2), true);
        assert_eq!(game.play(2), true);
        assert_eq!(game.play(2), true);
        assert_eq!(game.play(2), true);
        assert_eq!(game.play(2), true);
        assert_eq!(game.play(2), false);
        assert_eq!(game.play(3), true);
        assert_eq!(game.play(3), true);
        assert_eq!(game.play(3), true);
        assert_eq!(game.play(3), true);
        assert_eq!(game.play(3), true);
        assert_eq!(game.play(3), true);
        assert_eq!(game.play(3), false);
        assert_eq!(game.play(4), true);
        assert_eq!(game.play(4), true);
        assert_eq!(game.play(4), true);
        assert_eq!(game.play(4), true);
        assert_eq!(game.play(4), true);
        assert_eq!(game.play(4), true);
        assert_eq!(game.play(4), false);
        assert_eq!(game.play(5), true);
        assert_eq!(game.play(5), true);
        assert_eq!(game.play(5), true);
        assert_eq!(game.play(5), true);
        assert_eq!(game.play(5), true);
        assert_eq!(game.play(5), true);
        assert_eq!(game.play(5), false);
        assert_eq!(game.play(6), true);
        assert_eq!(game.play(6), true);
        assert_eq!(game.play(6), true);
        assert_eq!(game.play(6), true);
        assert_eq!(game.play(6), true);
        assert_eq!(game.play(6), true);
        assert_eq!(game.play(6), false);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut game = Connect4::new();
        game.play(0);
        game.play(1);
        game.play(2);
        game.play(3);
        assert_eq!(game.play(7), false);
        assert_eq!(game.play(8), false);
    }
}
