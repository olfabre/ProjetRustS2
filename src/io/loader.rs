use std::fs;
use serde_json;
use crate::models::{character::Character, dialogue::Dialogue, item::Item, pnj::Pnj, room::Room};

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

//charge le inventaires depui inventory.json
/*pub fn load_inventory_from_file(filename: &str) -> Result<Vec<Inventory>>{
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des Inventory.");
    let inventory: Vec<Inventory> = serde_json::from_str(&data)?;
    Ok(inventory)
}*/
