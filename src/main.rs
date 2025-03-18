mod models;
mod io;

use models::game::Game;

fn main() {

    let mut game = Game::new();
    game.run();
    
}
