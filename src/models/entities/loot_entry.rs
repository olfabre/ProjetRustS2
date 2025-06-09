use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LootEntry {
    pub item_id: u32,
    pub min_quantity: u32, // Minimum quantity of the item to drop
    pub max_quantity: u32, // Maximum quantity of the item to drop
    pub drop_chance: f32, // Chance of dropping the item (0.0 to 1.0)
}