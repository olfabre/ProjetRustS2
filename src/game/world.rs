use serde::{Serialize, Deserialize};
use crate::game::character::Character;
use crate::game::data_loader::load_game_data;

#[derive(Serialize, Deserialize)]
pub struct Zone {
    pub name: String,
    pub description: String,
    pub effect: String,
    pub requires_item: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct World {
    pub zones: Vec<Zone>,
}

impl World {
    pub fn load_from_json(file_path: &str) -> Self {
        let data = load_game_data(file_path);
        data.world
    }

    pub fn explore(&self, player: &mut Character) {
        for zone in &self.zones {
            println!("🌍 Vous explorez : {}", zone.name);
            println!("{}", zone.description);
        }
    }
}
