mod traits;
mod models;

use serde::{Deserialize, Serialize};
use std::fs;

use models::{Piece, Joueur};
use serde_json;
use crate::models::Oridentation;
use crate::traits::{Deplacable, ComportementJoueur, ComportementObjet};

mod game;


fn main() {
    println!("🎮 Bienvenue dans le RPG en mode texte !");

    // Démarrer le jeu
    game::run();
}