//Sauvegarde et chargement des données

use std::fs;
use serde::{Serialize, Deserialize};

pub fn save_game<T: Serialize>(data: &T, filename: &str) {
    let content = serde_json::to_string_pretty(data).unwrap();
    fs::write(filename, content).expect("Erreur de sauvegarde !");
}

pub fn load_game<T: for<'de> Deserialize<'de>>(filename: &str) -> T {
    let content = fs::read_to_string(filename).expect("Impossible de lire le fichier");
    serde_json::from_str(&content).expect("Erreur de parsing JSON")
}
