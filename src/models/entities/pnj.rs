// Module de gestion des personnages non-joueurs (PNJ)
// Définit la structure Pnj et ses fonctionnalités
// Permet de gérer les interactions avec les PNJ, leurs dialogues et leur commerce

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

// Structure représentant un personnage non-joueur
// Hérite des propriétés de base d'un Vivant et ajoute des fonctionnalités spécifiques
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pnj {
    vivant: Vivant,        // Propriétés de base (santé, inventaire, etc.)
    pub role: String,      // Rôle du PNJ (marchand, quêteur, etc.)
    pub money: i32,        // Argent possédé par le PNJ
    pub dialogue_id: u32,  // ID du dialogue associé au PNJ
}

impl Pnj {
    // Gère l'interaction avec un PNJ
    // Permet d'initier un dialogue et de gérer les quêtes
    // Vérifie si le PNJ est présent dans la salle actuelle
    pub fn parler_au_pnj(pnj_nom: &str, character: &mut Character,
                         rooms: &[Room], pnjs: &mut [Pnj],
                         dialogues: &mut [Dialogue],
                         quetes: &mut HashMap<u32, Quete>, items: &Vec<Item>) {

        let room = &rooms[character.position];

        if let Some(pnj) = pnjs.iter_mut().find(|p| {
            room.pnjs.contains(&p.id()) && p.name().to_lowercase() == pnj_nom.to_lowercase()
        }) {
            if let Some(dialogue) = dialogues.iter_mut().find(|d| d.dialogue_id == pnj.dialogue_id) {
                dialogue.afficher_dialogue(character, quetes, items, pnj);
            } else {
                println!("💬 {} : \"Je n'ai rien à te dire.\"", pnj.name());
            }
        } else {
            println!("❌ Il n'y a pas de {} ici.", pnj_nom);
        }
    }

    // Retourne l'identifiant unique du PNJ
    pub fn id(&self) -> u32 {
        self.vivant.id()
    }

    // Retourne le nom du PNJ
    pub fn name(&self) -> &str {
        self.vivant.name()
    }

    // Retourne la description du PNJ
    pub fn description(&self) -> &str {
        self.vivant.description()
    }

    // Retourne une référence mutable à l'inventaire du PNJ
    pub fn inventory_mut(&mut self) -> &mut Inventory {
        self.vivant.inventory_mut()
    }
}

// Implémentation du trait MoneyManager pour Pnj
// Permet de gérer l'argent du PNJ (pour le commerce)
impl MoneyManager for Pnj {
    fn money_mut(&mut self) -> &mut i32 {
        &mut self.money
    }
}