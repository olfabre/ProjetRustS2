use serde::{Deserialize, Serialize}; 

use crate::models::room::Room;
use crate::models::dialogue::Dialogue;
use crate::models::elements::item::Item;
use crate::models::elements::vivant::Vivant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pnj {
    vivant: Vivant,
    role: String,
    dialogue_id: u32,
}

impl Pnj {
    /// Permet à un PNJ de parler en utilisant `dialogue.rs`
    /*pub fn parler_au_pnj(pnj_nom: &str, position: usize, rooms: &[Room], pnjs: &[Pnj], dialogues: &[Dialogue]) {
        let room = &rooms[position];

        if let Some(&pnj_id) = room.npcs.iter().find(|&&id| {
            pnjs.iter().any(|p| p.id == id && p.name.to_lowercase() == pnj_nom.to_lowercase())
        }) {
            if let Some(pnj) = pnjs.iter().find(|p| p.id == pnj_id) {
                if let Some(dialogue) = dialogues.iter().find(|d| d.dialogue_id == pnj.dialogue_id) {
                    dialogue.afficher_dialogue();
                } else {
                    println!("💬 {} : \"Je n’ai rien à te dire.\"", pnj.name);
                }
            }
        } else {
            println!("❌ Il n'y a pas de {} ici.", pnj_nom);
        }
    }*/

    pub fn parler_au_pnj(pnj_nom: &str, position: usize, rooms: &[Room], pnjs: &[Pnj], dialogues: &[Dialogue]) {
        let room = &rooms[position];

        if let Some(&pnj_id) = room.pnjs.iter().find(|&&id| {
            pnjs.iter().any(|p| p.id() == id && p.name().to_lowercase() == pnj_nom.to_lowercase())
        }) {
            if let Some(pnj) = pnjs.iter().find(|p| p.id() == pnj_id) {
                if let Some(dialogue) = dialogues.iter().find(|d| d.dialogue_id == pnj.dialogue_id) {
                    dialogue.afficher_dialogue();
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
        &self.vivant.name()
    }

    pub fn description(&self) -> &str {
        &self.vivant.description()
    }

    pub fn health(&self) -> u32 {
        self.vivant.health()
    }

    pub fn strength(&self) -> u32 {
        self.vivant.strength()
    }

    pub fn intelligence(&self) -> u32 {
        self.vivant.intelligence()
    }

    // pub fn inventory(&self) -> &Vec<Item> {
    //     &self.vivant.inventory()
    // }

    pub fn inventory(&mut self) -> &mut Vec<Item> {
        self.vivant.inventory()
    }

    pub fn set_name(&mut self, name: &str) {
        self.vivant.set_name(name);
    }

    pub fn set_id(&mut self, id: u32) {
        self.vivant.set_id(id);
    }

    pub fn set_description(&mut self, desc: &str) {
        self.vivant.set_description(desc);
    }

    pub fn set_health(&mut self, health: u32) {
        self.vivant.set_health(health);
    }

    pub fn set_strength(&mut self, strength: u32) {
        self.vivant.set_health(strength);
    }

    pub fn set_intelligence(&mut self, intelligence: u32) {
        self.vivant.set_intelligence(intelligence);
    }

    pub fn set_inventory(&mut self, inventory: Vec<Item>) {
        self.vivant.set_inventory(inventory);
    }
}
