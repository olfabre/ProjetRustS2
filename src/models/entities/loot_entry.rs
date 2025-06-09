// Module de gestion des tables de loot
// Définit la structure pour les objets qui peuvent être obtenus en récompense
// Utilisé pour générer aléatoirement les récompenses des ennemis vaincus

use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::models::entities::inventory_item::InventoryItem;


// Structure définissant un objet possible dans une table de loot
// Permet de spécifier les chances et quantités de drop pour chaque objet
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LootEntry {
    pub item_id: u32,        // Identifiant de l'objet qui peut être obtenu
    pub min_quantity: u32,   // Quantité minimale d'objets à obtenir
    pub max_quantity: u32,   // Quantité maximale d'objets à obtenir
    pub drop_chance: f32,    // Probabilité d'obtenir l'objet (entre 0.0 et 1.0)
}

impl LootEntry {
    pub fn generer_depuis_table(loot_table: &[LootEntry]) -> Vec<InventoryItem> {
        let mut rng = rand::thread_rng();
        let mut resultat = vec![];

        for entry in loot_table {
            if rng.gen_bool(entry.drop_chance as f64) {
                let qte = rng.gen_range(entry.min_quantity..=entry.max_quantity);
                resultat.push(InventoryItem {
                    item_id: entry.item_id,
                    quantity: qte,
                });
            }
        }

        resultat
    }
}