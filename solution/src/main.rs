use ultron::*;

fn main() {
    let mut game = Game::new();
    
    loop {
        turn(&mut game);
    }
}
