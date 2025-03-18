mod io;
mod models;

use models::game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
