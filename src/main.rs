// Point d'entrée principal du jeu
// Définit les modules utilisés et initialise le jeu

// Import des modules principaux du jeu
mod models;  // Module contenant la logique métier du jeu
mod io;      // Module gérant les entrées/sorties et le chargement des données

use models::game::Game;  // Import de la structure Game qui gère le jeu

// Fonction principale qui initialise et lance le jeu
fn main() {
    // Création d'une nouvelle instance du jeu
    let mut game = Game::new();
    // Démarrage de la boucle principale du jeu
    game.run();
}
