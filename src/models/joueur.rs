
use serde::{Serialize, Deserialize};
use crate::models::Oridentation;
use crate::models::piece::Piece;
use crate::traits::{ComportementJoueur, ComportementObjet, Deplacable};

#[derive(Debug, Serialize, Deserialize)]
pub struct Joueur {
    pub r#type: String,
    pub nom: String,
    pub description: String,
    pub position: String,
    pub vie: u32,
    pub force: u32,
}



impl ComportementJoueur for Joueur {
    fn afficher_status(&self) {
        println!(
            "Joueur: {} est à la position {} avec {} de vie et {} de force",
            self.nom, self.position, self.vie, self.force
        );
    }
}

impl ComportementObjet for Joueur {
    fn get_type(&self) -> &str {
        &self.r#type
    }

    fn decriver(&self) {
        println!(
            "{}",
            self.description
        );
    }
}

impl Deplacable for Joueur {
    fn avancer(&mut self, pieces: &Vec<Piece>, direction: Oridentation) {
        // Find the current piece where the player is located
        if let Some(piece_actuelle) = pieces.iter().find(|p| p.id == self.position) {
            // Check if the piece has a connection in the desired direction
            if let Some(connexion) = piece_actuelle.connections.iter().find(|c| c.orientation == direction) {
                self.position = connexion.destination.clone();
                println!("✅ {} avance vers {:?} et arrive dans la pièce {}", self.nom, direction, self.position);
            } else {
                println!("❌ Il n'y a pas de sortie dans cette direction !");
            }
        } else {
            println!("❌ Le joueur est dans une pièce inconnue !");
        }
    }
}
