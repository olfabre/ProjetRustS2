mod models;
mod io;
mod traits;

use models::game::Game;

fn main() {

    let mut game = Game::new();
    game.run();
    
}
