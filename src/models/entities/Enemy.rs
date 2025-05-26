
use serde::{Deserialize, Serialize};
use crate::models::entities::loot_entry::LootEntry;
use crate::models::entities::vivant::Vivant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Enemy {
    vivant: Vivant,
    pub aggressiveness: i32;
    pub loot_list: Vec<LootEntry>
}


impl Enemy {



    pub fn is_alive(&self) -> bool{

        self.health > 0
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}