use std::collections::HashMap;
use rand::Rng;

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

    
    let winner = board.is_someone_winning();
    if winner.is_some() {
        return match winner.unwrap() {
            crate::Player::Red => 100 * turn_multi,
            crate::Player::Yellow => -100 * turn_multi,
        };
    }
    else {
        return better_evaluate(board);
    }
        

    
}

fn better_evaluate(board: &Connect4) -> i32 {
    let mut score = 0;
    let mut count = 0;
    let turn = board.get_turn();
    let opponent = match turn {
        crate::Player::Red => crate::Player::Yellow,
        crate::Player::Yellow => crate::Player::Red,
    };

    // Horizontal
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

    // Vertical
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

    // Diagonal
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

    let (threat, threat_list) = count_threat(board);
    // threat worth 10 points
    score += threat * 10;

    if is_threat_one_above_another(threat_list) {
        score += 20;
    }

    score as i32
}

fn is_threat_one_above_another(threat_list: Vec<Threat>) -> bool {
    for i in 0..threat_list.len() {
        for j in i+1..threat_list.len() {
            if threat_list[i].position.1 == threat_list[j].position.1 && threat_list[i].position.0 == threat_list[j].position.0 - 1 {
                return true;
            }
        }
    }
    false
}

pub fn find_best_move(board: &mut Connect4, depth: i32) -> u32 {
    let mut position_history: HashMap<u64, i32> = HashMap::new();
    let mut best_move = 0;
    let mut best_value = -10000;

    for i in 0..board.get_size().width {
        if board.play(i) {
            let value = -alpha_beta_pruning(board, depth - 1, -10000, 10000, &mut position_history);
            board.undo().unwrap();
            println!("Move {} has value {}", i, value);
            if value > best_value {
                best_value = value;
                best_move = i;
            }
        }
    }

    if best_value <= -100 {
        best_move = rand::thread_rng().gen_range(0..board.get_size().width);
    }

    best_move
}

fn alpha_beta_pruning(board: &mut Connect4, depth: i32, mut alpha: i32, beta: i32, position_history: &mut HashMap<u64, i32>) -> i32 {
    let score = evaluate_board(board);

    position_history.insert(board.get_hash(), score);

    if depth == 0 || score == 100 || score == -100 {
        return score;
    }
    
    for i in 0..board.get_size().width {
        if board.play(i) {
            let value = if position_history.contains_key(&board.get_hash()) {
                *position_history.get(&board.get_hash()).unwrap()
            } else {
                -alpha_beta_pruning(board, depth - 1, -beta, -alpha, position_history)
            };
            
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


struct Threat {
    position: (u32, u32),
}

/// Count the number of threats on the board
/// A threat is a sequence of 3 pieces of the same color with an empty cell at the end
fn count_threat(board: &Connect4) -> (u32, Vec<Threat>) {
    let mut count = 0;
    let mut threat = 0;
    let mut empty = 0;
    let mut last = crate::Player::Red;
    let mut threat_list = Vec::new();

    // Horizontal threats
    for i in 0..board.get_size().height {
        for j in 0..board.get_size().width {
            if board.get_cell(i, j) == Some(&last) {
                threat += 1;
            } else if board.get_cell(i, j) == None {
                empty += 1;
            } else {
                last = match last {
                    crate::Player::Red => crate::Player::Yellow,
                    crate::Player::Yellow => crate::Player::Red,
                };
                if threat == 3 && empty == 1 {
                    threat_list.push(Threat { position: (i, j) });
                    count += 1;
                }
                threat = 1;
                empty = 0;
            }
        }
        if threat == 3 && empty == 1 {
            count += 1;
            threat_list.push(Threat { position: (i, board.get_size().width) });
        }
        threat = 0;
        empty = 0;
    }

    // Vertical threats
    for j in 0..board.get_size().width {
        for i in 0..board.get_size().height {
            if board.get_cell(i, j) == Some(&last) {
                threat += 1;
            } else if board.get_cell(i, j) == None {
                empty += 1;
            } else {
                last = match last {
                    crate::Player::Red => crate::Player::Yellow,
                    crate::Player::Yellow => crate::Player::Red,
                };
                if threat == 3 && empty == 1 {
                    count += 1;
                    threat_list.push(Threat { position: (i, j) });
                }
                threat = 1;
                empty = 0;
            }
        }
        if threat == 3 && empty == 1 {
            count += 1;
            threat_list.push(Threat { position: (board.get_size().height, j) });
        }
        threat = 0;
        empty = 0;
    }

    // Diagonal threats
    for i in 0..board.get_size().height {
        for j in 0..board.get_size().width {
            if i + 3 < board.get_size().height && j + 3 < board.get_size().width {
                for k in 0..4 {
                    if board.get_cell(i + k, j + k) == Some(&last) {
                        threat += 1;
                    } else if board.get_cell(i + k, j + k) == None {
                        empty += 1;
                    } else {
                        last = match last
                        {
                            crate::Player::Red => crate::Player::Yellow,
                            crate::Player::Yellow => crate::Player::Red,
                        };
                        if threat == 3 && empty == 1 {
                            count += 1;
                            threat_list.push(Threat { position: (i + k, j + k) });
                        }
                        threat = 1;
                        empty = 0;
                    }
                }
                if threat == 3 && empty == 1 {
                    count += 1;
                    threat_list.push(Threat { position: (i + 3, j + 3) });
                }
                threat = 0;
                empty = 0;
            }
            if i + 3 < board.get_size().height && j >= 3 {
                for k in 0..4 {
                    if board.get_cell(i + k, j - k) == Some(&last) {
                        threat += 1;
                    } else if board.get_cell(i + k, j - k) == None {
                        empty += 1;
                    } else {
                        last = match last
                        {
                            crate::Player::Red => crate::Player::Yellow,
                            crate::Player::Yellow => crate::Player::Red,
                        };
                        if threat == 3 && empty == 1 {
                            count += 1;
                            threat_list.push(Threat { position: (i + k, j - k) });
                        }
                        threat = 1;
                        empty = 0;
                    }
                }
                if threat == 3 && empty == 1 {
                    count += 1;
                    threat_list.push(Threat { position: (i + 3, j - 3) });
                }
                threat = 0;
                empty = 0;
            }
        }
    }

    (count, threat_list)
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