use ultron::*;

fn main() {
    let mut game = Game::new();
    
    loop {
        turn(&mut game);
        if game.turn_number < 2 {
            game.turn_number += 1;            
        } else {
            game.turn_number = 0;
        }
    }
}
