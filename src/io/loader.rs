use std::{fs, io};
use std::io::{stdout, Write};
use serde_json;
use std::collections::HashMap;

use crate::models::{
    entities::character::Character,
    dialogue::Dialogue,
    entities::item::Item,
    entities::pnj::Pnj,
    entities::room::Room,
    entities::Enemy::Enemy,
    entities::quete::Quete,
};

/// Charge les salles depuis un fichier JSON contenant des RoomWrapper
pub fn load_room_from_file(filename: &str) -> Result<Vec<Room>, serde_json::Error> {
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des zones");
    let rooms: Vec<Room> = serde_json::from_str(&data)?;
    Ok(rooms)
}


pub fn load_characters_from_file(filename: &str) -> Result<Vec<Character>, serde_json::Error> {
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des personnages.");
    let characters: Vec<Character> = serde_json::from_str(&data)?;
    Ok(characters)
}

pub fn load_items_from_file(filename: &str) -> Result<Vec<Item>, serde_json::Error> {
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des objets.");
    let items: Vec<Item> = serde_json::from_str(&data)?;
    Ok(items)
}

/// Charge les PNJ depuis `pnj.json`
pub fn load_pnjs_from_file(filename: &str) -> Result<Vec<Pnj>, serde_json::Error> {
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des PNJ.");
    let pnjs: Vec<Pnj> = serde_json::from_str(&data)?;
    Ok(pnjs)
}

/// Charge les dialoque depuis `dialogue.json`
pub fn load_dialogues_from_file(filename: &str) -> Result<Vec<Dialogue>, serde_json::Error> {
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des PNJ.");
    let dialogues: Vec<Dialogue> = serde_json::from_str(&data)?;
    Ok(dialogues)
}

pub fn load_ennemie_from_file(filename: &str) -> Result<Vec<Enemy>, serde_json::Error>{
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des Ennemie.");
    let ennemie: Vec<Enemy> = serde_json::from_str(&data)?;
    Ok(ennemie)
}

/*pub fn load_quetes_from_file(filename: &str) -> Result<Vec<Quete>, serde_json::Error>{
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des Quêtes.");
    let quetes: Vec<Quete> = serde_json::from_str(&data)?;
    Ok(quetes)
}*/



pub fn load_quetes_from_file(filename: &str) -> Result<HashMap<u32, Quete>, serde_json::Error> {
    // Read the file contents into a string.
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des Quêtes.");

    // Deserialize the JSON into a Vec<Quete>.
    let quetes: Vec<Quete> = serde_json::from_str(&data)?;

    // Create a HashMap from the Vec<Quete>, mapping IDs to Quete structs.
    let quete_map: HashMap<u32, Quete> = quetes
        .into_iter()
        .map(|quete| (quete.id(), quete)) // Use the quest ID as the key.
        .collect();

    // Return the resulting HashMap.
    Ok(quete_map)
}


pub fn get_user_input() -> String {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Erreur de lecture.");
    input
}

//charge le inventaires depui inventory.json
/*pub fn load_inventory_from_file(filename: &str) -> Result<Vec<Inventory>>{
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des Inventory.");
    let inventory: Vec<Inventory> = serde_json::from_str(&data)?;
    Ok(inventory)
}*/
