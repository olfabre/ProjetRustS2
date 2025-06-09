use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::models::entities::room::Room;
use crate::models::dialogue::Dialogue;
use crate::models::entities::character::Character;
use crate::models::entities::inventory::Inventory;
use crate::models::entities::item::Item;
use crate::models::entities::quete::Quete;
use crate::models::entities::vivant::Vivant;
use crate::models::traits::money_manager::MoneyManager;

// Structure qui représente un personnage non-joueur (PNJ) dans le jeu
// Hérite des attributs de base d'une entité vivante et ajoute des fonctionnalités de dialogue et de commerce
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pnj {
    vivant: Vivant,           // Attributs de base d'une entité vivante (stats, inventaire, etc.)
    pub role: String,         // Rôle ou fonction du PNJ dans le jeu
    pub money: i32,           // Argent que possède le PNJ pour le commerce
    pub dialogue_id: u32,     // ID du dialogue associé au PNJ
}

impl Pnj {
    /// Permet au joueur d'interagir avec un PNJ
    /// Affiche le dialogue approprié et gère les interactions possibles
    pub fn parler_au_pnj(pnj_nom: &str, character: &mut Character,
                         rooms: &[Room], pnjs: &mut [Pnj],
                         dialogues: &mut [Dialogue],
                         quetes: &mut HashMap<u32, Quete>, items: &Vec<Item>) {
        // Récupère la salle actuelle du joueur
        let room = &rooms[character.position];

        // Recherche le PNJ dans la salle actuelle
        if let Some(pnj) = pnjs.iter_mut().find(|p| {
            room.pnjs.contains(&p.id()) && p.name().to_lowercase() == pnj_nom.to_lowercase()
        }) {
            // Recherche et affiche le dialogue associé au PNJ
            if let Some(dialogue) = dialogues.iter_mut().find(|d| d.dialogue_id == pnj.dialogue_id) {
                dialogue.afficher_dialogue(character, quetes, items, pnj);
            } else {
                println!("💬 {} : \"Je n'ai rien à te dire.\"", pnj.name());
            }
        } else {
            println!("❌ Il n'y a pas de {} ici.", pnj_nom);
        }
    }

    // Getters pour les attributs de base
    pub fn id(&self) -> u32 {
        self.vivant.id()
    }

    pub fn name(&self) -> &str {
        self.vivant.name()
    }

    pub fn description(&self) -> &str {
        self.vivant.description()
    }

    // Getter mutable pour l'inventaire
    pub fn inventory_mut(&mut self) -> &mut Inventory {
        self.vivant.inventory_mut()
    }
}

// Implémentation du trait MoneyManager pour gérer l'argent du PNJ
impl MoneyManager for Pnj {
    // Retourne une référence mutable à l'argent du PNJ
    fn money_mut(&mut self) -> &mut i32 {
        &mut self.money
    }
}