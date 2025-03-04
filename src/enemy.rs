use serde::{Serialize, Deserialize};
use crate::game::data_loader::load_game_data;

#[derive(Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub health: i32,
    pub strength: i32,
    pub agility: i32,
}

impl Enemy {
    pub fn load_from_json() -> Vec<Self> {
        let data = load_game_data();
        data.enemies
    }
}
