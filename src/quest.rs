use serde::{Serialize, Deserialize};
use crate::game::item::Item;
use crate::game::data_loader::load_game_data;

#[derive(Serialize, Deserialize, Clone)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub reward: String,
    pub completed: bool,
}

impl Quest {
    pub fn load_from_json(file_path: &str) -> Vec<Self> {
        let data = load_game_data(file_path);
        data.quests
    }
}
