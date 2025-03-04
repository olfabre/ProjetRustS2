use serde::{Deserialize, Serialize};
use std::fs;

const GAME_DATA_FILE: &str = "../data/game_data.json"; // Chemin du fichier JSON // Définition du chemin du fichier JSON

#[derive(Serialize, Deserialize)]
struct GameData {
    characters: Vec<Character>,
    enemies: Vec<Enemy>,
    items: Vec<Item>,
    quests: Vec<Quest>,
    world: World,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub health: i32,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub level: i32,
    pub experience: i32,
    pub inventory: Vec<Item>,
    pub active_quests: Vec<Quest>,
    pub completed_quests: Vec<Quest>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Enemy {
    pub name: String,
    pub health: i32,
    pub strength: i32,
    pub agility: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ItemType {
    Potion,
    Weapon,
    Armor,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub effect: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub reward: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Zone {
    pub name: String,
    pub description: String,
    pub npcs: Vec<Npc>,
    pub items: Vec<Item>,
    pub effect: String,
    pub requires_item: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct World {
    pub zones: Vec<Zone>,
}

#[derive(Serialize, Deserialize)]
pub struct Npc {
    pub name: String,
    pub role: String,
    pub dialogue: Vec<String>,
    pub quests: Vec<Quest>,
}

pub fn load_game_data() -> GameData {
    let data = fs::read_to_string(GAME_DATA_FILE).expect("Impossible de lire le fichier JSON");
    serde_json::from_str(&data).expect("Erreur de parsing JSON")
}
