use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Piece {
    r#type: String,  // "Piece"
    id: String,
    nom: String,
    description: String,
    connections: Vec<Connection>,
    inventaire: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Joueur {
    r#type: String,  // "Joueur"
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

fn main() {
    // Lire le fichier JSON
    let json_str = fs::read_to_string("src/data.json").expect("Impossible de lire le fichier");

    // Parser le JSON en une liste générique de `serde_json::Value`
    let objets: Vec<serde_json::Value> = serde_json::from_str(&json_str).expect("Erreur de parsing JSON");

    // Parcourir les objets JSON et les parser dans leurs structures respectives
    for obj in &objets {
        if obj["type"] == "Joueur" {
            let joueur: Joueur = serde_json::from_value(obj.clone()).expect("Erreur de parsing Joueur");
            println!("Joueur : {:?}", joueur);
        } else if obj["type"] == "Piece" {
            let piece: Piece = serde_json::from_value(obj.clone()).expect("Erreur de parsing Piece");
            println!("Pièce : {:?}", piece);
        } else {
            println!("Type inconnu : {:?}", obj);
        }
    }
}
