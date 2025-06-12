
use serde::{Deserialize, Serialize};

use crate::models::entities::character::Character;
use crate::models::entities::inventory::Inventory;
use crate::models::entities::vivant::Vivant;
use crate::models::game::Game;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pnj {
    vivant: Vivant,
    pub role: String,
    pub money: i32,
    pub dialogue_id: u32, // reference to dialogue
}

impl Pnj {

    // Permet à un PNJ de parler en utilisant `dialogue.rs`
    pub fn parler_au_pnj(pnj_nom: &str, character: &mut Character, game: &mut Game) {

        let room = &game.rooms[character.position];

        if let Some(pnj) = game.pnjs.iter_mut().find(|p| {
            room.pnjs.contains(&p.id()) && p.name().to_lowercase() == pnj_nom.to_lowercase()
        }) {
            if let Some(dialogue) = game.dialogues.iter_mut().find(|d| d.dialogue_id == pnj.dialogue_id) {
                dialogue.afficher_dialogue(character, &mut game.quetes, &mut game.items, pnj);
            } else {
                println!("💬 {} : \"Je n’ai rien à te dire.\"", pnj.name());
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

    pub fn inventory_mut(&mut self) -> &mut Inventory {
        self.vivant.inventory_mut()
    }

}

