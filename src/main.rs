use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")] // Differencie automatiquement "Joueur" et "Piece"
enum Objet {
    Piece {
        id: String,
        nom: String,
        description: String,
        connections: Vec<Connection>,
        inventaire: Vec<String>,
    },
    Joueur {
        nom: String,
        position: String,
        vie: u32,
        force: u32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Connection {
    orientation: String,
    destination: String,
}

fn main() {
    // Lire le fichier JSON
    let json_str = fs::read_to_string("src/data.json").expect("Impossible de lire le fichier");

    // Parser le JSON en Vec<Objet>
    let objets: Vec<Objet> = serde_json::from_str(&json_str).expect("Erreur de parsing JSON");

    // Afficher les objets
    for obj in &objets {
        match obj {
            Objet::Joueur { nom, position, vie, force } => {
                println!("Joueur trouvé : {} à la position {} avec {} de vie et {} de force.",
                         nom, position, vie, force);
            }
            Objet::Piece { id, nom, description, connections, inventaire } => {
                println!("Pièce {} ({}) : {} - Connexions: {:?}", id, nom, description, connections);
            }
        }
    }
}
