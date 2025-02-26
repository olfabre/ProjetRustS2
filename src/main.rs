use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Piece {
    r#type: String,
    id: String,
    nom: String,
    description: String,
    connections: Vec<Connection>,
    inventaire: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Joueur {
    r#type: String,
    nom: String,
    position: String,
    vie: u32,
    force: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Connection {
    orientation: String,
    destination: String,
}

// Trait pour afficher les objets du jeu
trait GameObject {
    fn get_type(&self) -> &str;
    fn afficher(&self);
}

impl GameObject for Piece {
    fn get_type(&self) -> &str {
        &self.r#type
    }

    fn afficher(&self) {
        println!("Pièce: {} ({}) - {}", self.id, self.nom, self.description);
    }
}

impl GameObject for Joueur {
    fn get_type(&self) -> &str {
        &self.r#type
    }

    fn afficher(&self) {
        println!(
            "Joueur: {} est à la position {} avec {} de vie et {} de force",
            self.nom, self.position, self.vie, self.force
        );
    }
}

// Nouveau Trait pour permettre au joueur de se déplacer
trait Deplacable {
    fn avancer_nord(&mut self, pieces: &Vec<Piece>);
}

impl Deplacable for Joueur {
    fn avancer_nord(&mut self, pieces: &Vec<Piece>) {
        // Trouver la pièce actuelle du joueur
        if let Some(piece_actuelle) = pieces.iter().find(|p| p.id == self.position) {
            // Vérifier si une connexion au nord existe
            if let Some(connexion) = piece_actuelle.connections.iter().find(|c| c.orientation == "nord") {
                self.position = connexion.destination.clone();
                println!("✅ {} avance vers le nord et arrive dans la pièce {}", self.nom, self.position);
            } else {
                println!("❌ Il n'y a pas de sortie au nord !");
            }
        } else {
            println!("❌ Le joueur est dans une pièce inconnue !");
        }
    }
}

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

    // Vérifier si un joueur existe et lui faire avancer vers le nord
    if let Some(ref mut joueur) = joueur {
        joueur.afficher();
        joueur.avancer_nord(&pieces);
        joueur.afficher(); // Afficher après déplacement
    } else {
        println!("❌ Aucun joueur trouvé dans le JSON !");
    }
}
