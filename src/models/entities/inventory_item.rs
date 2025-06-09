// Module définissant la structure d'un item dans l'inventaire
// Représente un objet avec sa quantité dans l'inventaire

use serde::{Deserialize, Serialize};

// Structure représentant un objet dans l'inventaire
// Stocke l'identifiant de l'objet et sa quantité
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item_id: u32,  // Identifiant unique de l'objet
    pub quantity: u32, // Quantité de l'objet dans l'inventaire
}