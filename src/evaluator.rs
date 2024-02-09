use crate::Connect4;

/// Evaluate the board and return a score
/// If the player is winning, the score is positive +100
/// If the opponent is winning, the score is negative -100
/// If no one is winning, the score is 0
fn evaluate_board(board: &Connect4) -> i32 {
    let turn_multi = match board.get_turn() {
        crate::Player::Red => 1,
        crate::Player::Yellow => -1,
    };

    for _i in 0..board.get_size().height {
        for _j in 0..board.get_size().width {
            
            let winner = board.is_someone_winning();
            if winner.is_some() {
                return match winner.unwrap() {
                    crate::Player::Red => 100 * turn_multi,
                    crate::Player::Yellow => -100 * turn_multi,
                };
            } else {
                return better_evaluate(board);
            }
        }
    }

    0
}

fn better_evaluate(board: &Connect4) -> i32 {
    let mut score = 0;
    let mut count = 0;
    let turn = board.get_turn();
    let opponent = match turn {
        crate::Player::Red => crate::Player::Yellow,
        crate::Player::Yellow => crate::Player::Red,
    };

    for i in 0..board.get_size().height {
        for j in 0..board.get_size().width {
            if board.get_cell(i, j) == Some(turn) {
                count += 1;
            } else if board.get_cell(i, j) == Some(&opponent) {
                count = 0;
                break;
            }
        }
        score += count;
        count = 0;
    }

    for j in 0..board.get_size().width {
        for i in 0..board.get_size().height {
            if board.get_cell(i, j) == Some(turn) {
                count += 1;
            } else if board.get_cell(i, j) == Some(&opponent) {
                count = 0;
                break;
            }
        }
        score += count;
        count = 0;
    }

    for i in 0..board.get_size().height {
        for j in 0..board.get_size().width {
            if i + 3 < board.get_size().height && j + 3 < board.get_size().width {
                for k in 0..4 {
                    if board.get_cell(i + k, j + k) == Some(turn) {
                        count += 1;
                    } else if board.get_cell(i + k, j + k) == Some(&opponent) {
                        count = 0;
                        break;
                    }
                }
                score += count;
                count = 0;
            }
            if i + 3 < board.get_size().height && j >= 3 {
                for k in 0..4 {
                    if board.get_cell(i + k, j - k) == Some(turn) {
                        count += 1;
                    } else if board.get_cell(i + k, j - k) == Some(&opponent) {
                        count = 0;
                        break;
                    }
                }
                score += count;
                count = 0;
            }
        }
    }

    score
}

pub fn find_best_move(board: &mut Connect4, depth: i32) -> u32 {
    let mut best_move = 8;
    let mut best_value = -1000;

    for i in 0..board.get_size().width {
        if board.play(i) {
            let move_value = -alpha_beta_pruning(board, depth - 1, -1000, 1000);
            println!("Move: {} Value: {}", i, move_value);
            board.undo().unwrap();
            if move_value > best_value {
                best_move = i;
                best_value = move_value;
            }
        }
    }

    best_move
}

fn alpha_beta_pruning(board: &mut Connect4, depth: i32, mut alpha: i32, beta: i32) -> i32 {
    let score = evaluate_board(board);

    if depth == 0 || score == 100 || score == -100 {
        return score;
    }
    
    for i in 0..board.get_size().width {
        if board.play(i) {
            let value = -alpha_beta_pruning(board, depth - 1, -beta, -alpha);
            board.undo().unwrap();
            if value >= beta {
                return beta;
            }
            if value > alpha {
                alpha = value;
            }
        }
    }

    alpha
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_board_winning() {
        let mut game = Connect4::new();
        game.play(0);
        game.play(0);
        game.play(1);
        game.play(1);
        game.play(2);
        game.play(2);
        game.play(3);
        game.play(0);
        assert_eq!(evaluate_board(&game), 100);
    }

    #[test]
    fn test_evaluate_board_losing() {
        let mut game = Connect4::new();
        game.play(0); // Red
        game.play(0); // Yellow
        game.play(1); // Red
        game.play(1); // Yellow
        game.play(2); // Red
        game.play(2); // Yellow

        game.play(4); // Red
        game.play(3); // Yellow
        game.play(4); // Red
        game.play(3); // Yellow
        assert_eq!(evaluate_board(&game), -100);
    }
}