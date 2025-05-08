use crate::evaluator::evaluate_position;
use crate::Connect4;
use actix_cors::Cors;
use actix_web::{
    delete, get, post, web, App, HttpResponse, HttpServer
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

struct GameAndDifficulty {
    game: Connect4,
    difficulty: i32,
}

// Store active games in a thread-safe HashMap
static GAMES: Lazy<Mutex<HashMap<String, GameAndDifficulty>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Serialize)]
struct GameResponse {
    id: String,
    board: Vec<Vec<String>>,
    turn: String,
    winner: Option<String>,
    is_draw: bool,
    last_move: Option<u32>, // Last move made by the player
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
}

#[derive(Deserialize)]
struct MoveRequest {
    column: u32,
    ai_difficulty: Option<i32>, // AI difficulty level
}

#[derive(Deserialize)]
struct NewGameRequest {
    ai_difficulty: Option<i32>,
}

#[derive(Deserialize)]
struct EvaluateRequest {
    depth: i32,
}

#[derive(Serialize)]
struct GameListResponse {
    games: Vec<String>,
}

// Convert game state to a serializable response
fn game_to_response(game: &Connect4, id: &str, last_move: Option<u32>) -> GameResponse {
    let board = game.get_board();
    let mut board_response = Vec::new();
    
    for row in board.s() {
        let mut row_response = Vec::new();
        for cell in row.iter() {
            match cell {
                crate::player::Player::Red => row_response.push("red".to_string()),
                crate::player::Player::Yellow => row_response.push("yellow".to_string()),
                crate::player::Player::Empty => row_response.push("empty".to_string()),
            }
        }
        board_response.push(row_response);
    }
    
    let winner = game.is_someone_winning().map(|player| match player {
        crate::player::Player::Red => "red".to_string(),
        crate::player::Player::Yellow => "yellow".to_string(),
    });
    
    GameResponse {
        id: id.to_string(),
        board: board_response,
        turn: match game.get_turn() {
            crate::player::Player::Red => "red".to_string(),
            crate::player::Player::Yellow => "yellow".to_string(),
        },
        winner,
        is_draw: game.is_draw(),
        last_move,
    }
}

// Create a new game
#[post("/games")]
async fn create_game(_req: web::Json<NewGameRequest>) -> HttpResponse {
    println!("Creating a new game...");
    let game = Connect4::new();
    let id = Uuid::new_v4().to_string();

    let difficulty = _req.ai_difficulty.unwrap_or(5);
    let mut games = GAMES.lock().unwrap();
    games.insert(id.clone(), GameAndDifficulty { game, difficulty });
    
    let game_ref = &games.get(&id).unwrap().game;
    HttpResponse::Created().json(game_to_response(game_ref, &id, None))
}

// Get game state
#[get("/games/{id}")]
async fn get_game(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    
    let games = GAMES.lock().unwrap();
    if let Some(game) = games.get(&id) {
        HttpResponse::Ok().json(game_to_response(&game.game, &id, None))
    } else {
        HttpResponse::NotFound().json(ErrorResponse {
            error: "Game not found".to_string(),
        })
    }
}

// List all active games
#[get("/games")]
async fn list_games() -> HttpResponse {
    let games = GAMES.lock().unwrap();
    let ids: Vec<String> = games.keys().cloned().collect();
    
    HttpResponse::Ok().json(GameListResponse { games: ids })
}

// Make a move
#[post("/games/{id}/move")]
async fn make_move(path: web::Path<String>, req: web::Json<MoveRequest>) -> HttpResponse {
    let id = path.into_inner();
    
    let mut games = GAMES.lock().unwrap();
    if let Some(game) = games.get_mut(&id) {
        if game.game.is_someone_winning().is_some() || game.game.is_draw() {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Game is already over".to_string(),
            });
        }
        
        if !game.game.play(req.column) {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid move".to_string(),
            });
        }
        
        // Check if the game is over after player's move
        let is_game_over = game.game.is_someone_winning().is_some() || game.game.is_draw();
        let mut last_move = None;
        // If game is not over, let AI make a move
        if !is_game_over {
            // Use the provided difficulty or default to game's default
            let difficulty = req.ai_difficulty.unwrap_or(game.difficulty);
            if difficulty < 0 {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid AI difficulty level, must be positive".to_string(),
                });
            }



            if difficulty != 0 {
                last_move = Some(game.game.play_minimax(difficulty));
            }
        }

        return HttpResponse::Ok().json(game_to_response(&game.game, &id, last_move));
    } 
    
    HttpResponse::NotFound().json(ErrorResponse {
        error: "Game not found".to_string(),
    })
}

// Delete a game
#[delete("/games/{id}")]
async fn delete_game(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    
    let mut games = GAMES.lock().unwrap();
    if games.remove(&id).is_some() {
        HttpResponse::Ok().json(SuccessResponse {
            success: true,
        })
    } else {
        HttpResponse::NotFound().json(ErrorResponse {
            error: "Game not found".to_string(),
        })
    }
}

// Get evaluation of the game state
#[get("/games/{id}/evaluate")]
async fn evaluate_game(path: web::Path<String>, req: web::Query<EvaluateRequest>) -> HttpResponse {
    let id = path.into_inner();
    
    let mut games = GAMES.lock().unwrap();
    if let Some(game) = games.get_mut(&id) {
        let depth = req.depth;
        let evaluation = evaluate_position(&mut game.game, depth);
        return HttpResponse::Ok().json(evaluation);
    }
    
    HttpResponse::NotFound().json(ErrorResponse {
        error: "Game not found".to_string(),
    })
}

pub async fn run_server() -> std::io::Result<()> {
    println!("Starting Connect4 server on http://0.0.0.0:8080");
    
    match HttpServer::new(|| {
        // Configure CORS to allow web clients to connect
        let cors = Cors::default()
            .allowed_origin("https://localhost:3000")
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://connect4.ozraam.uk")
            .allowed_origin("http://connect4.ozraam.uk")
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .wrap(cors)
            .service(create_game)
            .service(get_game)
            .service(list_games)
            .service(make_move)
            .service(delete_game)
            .service(evaluate_game)
    })
    .bind("0.0.0.0:8080") {
        Ok(server) => {
            println!("Server bound successfully to 0.0.0.0:8080");
            match server.run().await {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Server error: {}", e);
                    Err(e)
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to bind to 0.0.0.0:8080: {}", e);
            Err(e)
        }
    }
}