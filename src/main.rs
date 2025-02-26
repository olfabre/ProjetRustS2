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

// Définition du trait GameObject
trait GameObject {
    fn get_type(&self) -> &str;
    fn afficher(&self);
}

// Implémentation du trait pour Piece
impl GameObject for Piece {
    fn get_type(&self) -> &str {
        &self.r#type
    }

    fn afficher(&self) {
        println!(
            "Pièce: {} ({}) - {}",
            self.id, self.nom, self.description
        );
    }
}

// Implémentation du trait pour Joueur
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

fn main() {
    // Lire le fichier JSON
    let json_str = fs::read_to_string("src/data.json").expect("Impossible de lire le fichier");

    // Parser le JSON en une liste d'objets génériques
    let objets: Vec<serde_json::Value> = serde_json::from_str(&json_str).expect("Erreur de parsing JSON");

    // Liste pour stocker dynamiquement les objets qui implémentent GameObject
    let mut game_objects: Vec<Box<dyn GameObject>> = Vec::new();

    for obj in &objets {
        if obj["type"] == "Joueur" {
            let joueur: Joueur = serde_json::from_value(obj.clone()).expect("Erreur de parsing Joueur");
            game_objects.push(Box::new(joueur));
        } else if obj["type"] == "Piece" {
            let piece: Piece = serde_json::from_value(obj.clone()).expect("Erreur de parsing Piece");
            game_objects.push(Box::new(piece));
        }
    }

    // Affichage des objets dynamiquement
    for obj in &game_objects {
        obj.afficher();
    }
}
