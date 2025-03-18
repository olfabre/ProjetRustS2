use serde::{Deserialize, Serialize}; 

use crate::models::room::Room;
use crate::models::dialogue::Dialogue;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pnj {
    pub id: u32,
    pub name: String,
    pub role: String,
    pub dialogue_id: u32,
}

impl Pnj {
    /// 🔥 Permet à un PNJ de parler en utilisant `dialogue.rs`
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
    }
}
