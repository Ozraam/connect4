use std::error::Error;

use connect4::Connect4;

fn main() {
    let mut game = Connect4::new();
    game.print_board();

    loop {
        let play = match get_user_play() {
            Ok(play) => play,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        if game.play(play) {
            game.print_board();
            if let Some(player) = game.is_someone_winning() {
                println!("Player {:?} wins!", player);
                break;
            }
            if game.is_draw() {
                println!("It's a draw!");
                break;
            }
            game.play_minimax(8);
            game.print_board();
            if let Some(player) = game.is_someone_winning() {
                println!("Player {:?} wins!", player);
                break;
            }
            if game.is_draw() {
                println!("It's a draw!");
                break;
            }
        } else {
            println!("Invalid play");
        }
    }

    println!("Game over");
    println!("Press enter to exit");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn get_user_play() -> Result<u32, Box<dyn Error>> {
    println!("Enter a column number to play (0-6):");
    let mut play = String::new();
    std::io::stdin().read_line(&mut play)?;
    Ok(play.trim().parse()?)
}