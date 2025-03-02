mod traits;
mod models;

use serde::{Deserialize, Serialize};
use std::fs;

use models::{Piece, Joueur};
use serde_json;
use crate::models::Oridentation;
use crate::traits::{Deplacable, ComportementJoueur, ComportementObjet};

fn main() {
    // Lire le fichier JSON
    let json_str = fs::read_to_string("src/data.json").expect("Impossible de lire le fichier");

    // Parser le JSON en une liste d'objets
    let objets: Vec<serde_json::Value> = serde_json::from_str(&json_str).expect("Erreur de parsing JSON");

    let mut pieces: Vec<Piece> = Vec::new();
    let mut joueur: Option<Joueur> = None;

    // Trier les objets en `Piece` et `Joueur`
    for obj in objets {
        if obj["type"] == "Piece" {
            let piece: Piece = serde_json::from_value(obj.clone()).expect("Erreur de parsing Piece");
            pieces.push(piece);
        } else if obj["type"] == "Joueur" {
            let j: Joueur = serde_json::from_value(obj.clone()).expect("Erreur de parsing Joueur");
            joueur = Some(j);
        }
    }

    // Vérifier si un joueur existe et lui faire avancer dans une direction donnée
    if let Some(ref mut joueur) = joueur {
        joueur.afficher_status();
        joueur.avancer(&pieces, Oridentation::NORD); // Test moving north
        joueur.afficher_status(); // Afficher après déplacement
        // println!("{}", pieces.last().unwrap().description.to_string());
        pieces.last().unwrap().decriver();

    } else {
        println!("❌ Aucun joueur trouvé dans le JSON !");
    }
}
