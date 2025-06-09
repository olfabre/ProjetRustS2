// Structure représentant un élément dans l'inventaire
// Utilisée pour stocker les objets et leur quantité
// Permet la sérialisation/désérialisation pour la sauvegarde
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item_id: u32,    // Identifiant unique de l'objet
    pub quantity: u32,   // Nombre d'exemplaires de l'objet
}