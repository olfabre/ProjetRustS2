use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::models::entities::room::Room;
use crate::models::dialogue::Dialogue;
use crate::models::entities::character::Character;
use crate::models::entities::item::Item;
use crate::models::entities::quete::Quete;
use crate::models::entities::vivant::Vivant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pnj {
    vivant: Vivant,
    pub role: String,
    pub dialogue_id: u32, // reference to dialogue
}

impl Pnj {
    /// Permet à un PNJ de parler en utilisant `dialogue.rs`

    pub fn parler_au_pnj(pnj_nom: &str, character: &mut Character,
                         rooms: &[Room],
                         pnjs: &[Pnj],
                         dialogues: &mut [Dialogue],
                         quetes: &mut HashMap<u32,Quete>,
                         items: &Vec<Item>) {
        let room = &rooms[character.position];

        if let Some(&pnj_id) = room.pnjs.iter().find(|&&id| {
            pnjs.iter().any(|p| id == p.id()  &&  p.name().to_lowercase() == pnj_nom.to_lowercase())
        }) {

            if let Some(pnj) = pnjs.iter().find(|p| p.id() == pnj_id) {

                if let Some(dialogue) = dialogues.iter_mut().find(|d| d.dialogue_id == pnj.dialogue_id) {

                    dialogue.afficher_dialogue(character, quetes, items);
                } else {
                    println!("💬 {} : \"Je n’ai rien à te dire.\"", pnj.name());
                }
            }
        } else {
            println!("❌ Il n'y a pas de {} ici.", pnj_nom);
        }
    }

    pub fn id(&self) -> u32 {
        self.vivant.id()
    }

    pub fn name(&self) -> &str {
        self.vivant.name()
    }

    pub fn description(&self) -> &str {
        self.vivant.description()
    }

    pub fn inventory_mut(&mut self) -> &mut Vec<Item> {
        self.vivant.inventory_mut()
    }

}
