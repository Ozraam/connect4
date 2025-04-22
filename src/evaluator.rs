use std::collections::HashMap;
use rand::Rng;

use crate::Connect4;

// Added enum to represent node types for transposition table
#[derive(Clone, Copy, PartialEq)]
enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
}

// Added struct to store more detailed information in the transposition table
struct TranspositionEntry {
    score: i32,
    depth: i32,
    node_type: NodeType,
}

// Replace simple HashMap with a more structured transposition table
type TranspositionTable = HashMap<u64, TranspositionEntry>;

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

pub fn find_best_move(board: &mut Connect4, max_depth: i32) -> u32 {
    let mut best_move = 0;
    let mut position_history: TranspositionTable = HashMap::new();
    
    // Column ordering heuristic - prioritize center columns
    let column_order = [3, 2, 4, 1, 5, 0, 6];
    let mut death_moves = vec![];
    
    // Implement iterative deepening - start with low depth and progressively increase
    for depth in 1..=max_depth {
        let mut best_value = -10000;
        let mut local_best_move = 0;

        for &i in &column_order {
            if board.play(i) {
                let value = -alpha_beta_pruning(board, depth - 1, -10000, 10000, &mut position_history);
                board.undo().unwrap();
                
                if depth == max_depth {
                    println!("Move {} has value {}", i, value);
                }
                
                if value > best_value {
                    best_value = value;
                    local_best_move = i;
                }

                if value == -100 {
                    // If we find a losing move, add it to the death_moves list
                    death_moves.push(i);
                }
            }
        }
        
        best_move = local_best_move;
        
        // If we found a winning move, no need to search deeper
        if best_value >= 90 {
            println!("Found winning move at depth {}", depth);
            break;
        }
    }

    // Add a small amount of randomness to avoid predictable play
    if rand::thread_rng().gen_bool(0.05) && best_move != 3 {
        let random_column = column_order[rand::thread_rng().gen_range(0..3)];

        // check if random column is not a kill move
        if !death_moves.contains(&random_column) {
            // check if random column is valid
            if board.play(random_column) {
                board.undo().unwrap();
                best_move = random_column;
            }
        }
    }

    best_move
}

fn alpha_beta_pruning(board: &mut Connect4, depth: i32, mut alpha: i32, mut beta: i32, position_history: &mut TranspositionTable) -> i32 {
    // Add early return for draw condition
    if board.is_draw() {
        return 0;
    }
    
    // Check transposition table first for faster lookups
    let board_hash = board.get_hash();
    if let Some(entry) = position_history.get(&board_hash) {
        if entry.depth >= depth {
            match entry.node_type {
                NodeType::Exact => return entry.score,
                NodeType::LowerBound => alpha = alpha.max(entry.score),
                NodeType::UpperBound => beta = beta.min(entry.score),
            }
            
            if alpha >= beta {
                return entry.score;
            }
        }
    }

    let score = evaluate_board(board);
    if depth == 0 || score.abs() == 100 {
        // Store the result in the transposition table
        position_history.insert(board_hash, TranspositionEntry { 
            score, 
            depth, 
            node_type: NodeType::Exact 
        });
        return score;
    }
    
    // Add column ordering heuristic - prioritize center columns
    let column_order = [3, 2, 4, 1, 5, 0, 6]; // Center-first ordering
    
    let mut best_score = -1000;
    let original_alpha = alpha;
    
    for &i in &column_order {
        if board.play(i) {
            let value = -alpha_beta_pruning(board, depth - 1, -beta, -alpha, position_history);
            board.undo().unwrap();
            
            best_score = best_score.max(value);
            
            if best_score > alpha {
                alpha = best_score;
            }
            
            if alpha >= beta {
                // Store a lower bound in the transposition table
                position_history.insert(board_hash, TranspositionEntry { 
                    score: best_score, 
                    depth, 
                    node_type: NodeType::LowerBound 
                });
                return best_score;
            }
        }
    }

    // Store the result in the transposition table
    let node_type = if best_score <= original_alpha {
        NodeType::UpperBound
    } else {
        NodeType::Exact
    };
    
    position_history.insert(board_hash, TranspositionEntry { 
        score: best_score, 
        depth, 
        node_type 
    });
    
    best_score
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