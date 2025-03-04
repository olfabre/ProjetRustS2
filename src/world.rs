use serde::{Serialize, Deserialize};
use crate::character::Character;
use crate::data_loader::load_game_data;

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    pub zones: Vec<Zone>,
}

impl World {
    pub fn load_from_json() -> Self {
        let data = load_game_data();
        data.world
    }

    pub fn explore(&self, player: &mut Character) {
        for zone in &self.zones {
            println!("🌍 Vous explorez : {}", zone.name);
            println!("{}", zone.description);
        }
    }
}
