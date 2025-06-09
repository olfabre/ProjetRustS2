use serde::{Deserialize, Serialize};

// Structure qui définit un objet pouvant être laissé par un ennemi
// Spécifie les chances et les quantités possibles pour chaque objet
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LootEntry {
    pub item_id: u32,          // ID de l'objet qui peut être laissé
    pub min_quantity: u32,      // Quantité minimale d'objets à laisser
    pub max_quantity: u32,      // Quantité maximale d'objets à laisser
    pub drop_chance: f32,       // Probabilité de laisser l'objet (entre 0.0 et 1.0)
}