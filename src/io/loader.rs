use std::fs;
use serde_json::Result;
use crate::models::{room::Room, character::Character, item::Item};

pub fn load_room_from_file(filename: &str) -> Result<Vec<Room>>{
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des zones");
    let zones: Vec<Room> = serde_json::from_str(&data)?;
    Ok(zones)
}

pub fn load_characters_from_file(filename: &str) -> Result<Vec<Character>> {
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des personnages.");
    let characters: Vec<Character> = serde_json::from_str(&data)?;
    Ok(characters)
}

pub fn load_items_from_file(filename: &str) -> Result<Vec<Item>> {
    let data = fs::read_to_string(filename).expect("Impossible de lire le fichier des objets.");
    let items: Vec<Item> = serde_json::from_str(&data)?;
    Ok(items)
}