use std::fs::File;
use ultron::*;

fn main() {
    let file_name = "output.txt";
    let mut game = Game::new(&file_name);
    let _ = File::create(file_name).unwrap();
    
    loop {
        turn(&mut game);
        game.turn_number += 1;
    }
}
