
use serde::{Serialize, Deserialize};
use crate::models::entities::inventory_item::InventoryItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub capacity: usize,
    pub items: Vec<InventoryItem>,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: vec![],
        }
    }

    pub fn add_item(&mut self, item_id: u32, quantity: u32) -> bool {
        if self.items.len() >= self.capacity && !self.items.iter().any(|item| item.item_id == item_id) {
            return false; // Inventaire plein
        }

        if let Some(existing) = self.items.iter_mut().find(|item| item.item_id == item_id) {
            existing.quantity += quantity;
        } else {
            self.items.push(InventoryItem { item_id, quantity });
        }
        true
    }

    pub fn remove_item(&mut self, item_id: u32, quantity: u32) -> bool {
        if let Some(pos) = self.items.iter().position(|item| item.item_id == item_id) {
            if self.items[pos].quantity > quantity {
                self.items[pos].quantity -= quantity;
            } else {
                self.items.remove(pos);
            }
            true
        } else {
            false
        }
    }

    pub fn get_quantity(&self, item_id: u32) -> u32 {
        self.items.iter().find(|item| item.item_id == item_id).map_or(0, |item| item.quantity)
    }
}