use connect4::Connect4;

fn main() {
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

    println!("{}", game);
}
