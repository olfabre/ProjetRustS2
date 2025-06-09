// Module de gestion de l'inventaire
// Permet de gérer la collection d'objets d'une entité avec une capacité limitée

use serde::{Serialize, Deserialize};
use crate::models::entities::inventory_item::InventoryItem;

// Structure représentant l'inventaire d'une entité
// Gère le stockage et la manipulation des objets avec une capacité maximale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub capacity: usize,           // Capacité maximale de l'inventaire
    pub items: Vec<InventoryItem>, // Liste des objets dans l'inventaire
}

impl Inventory {
    // Crée un nouvel inventaire avec une capacité spécifiée
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: vec![],
        }
    }

    // Ajoute un objet à l'inventaire
    // Retourne true si l'ajout a réussi, false si l'inventaire est plein
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

    // Retire un objet de l'inventaire
    // Retourne true si le retrait a réussi, false si l'objet n'existe pas
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

    // Obtient la quantité d'un objet spécifique dans l'inventaire
    // Retourne 0 si l'objet n'existe pas
    pub fn get_quantity(&self, item_id: u32) -> u32 {
        self.items.iter().find(|item| item.item_id == item_id).map_or(0, |item| item.quantity)
    }
}