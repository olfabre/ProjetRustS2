use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item_id: u32,
    pub quantity: u32,
}