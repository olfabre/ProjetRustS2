use serde::{Serialize, Deserialize};
use crate::game::data_loader::load_game_data;

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

impl Item {
    pub fn load_from_json(file_path: &str) -> Vec<Self> {
        let data = load_game_data(file_path);
        data.items
    }
}
