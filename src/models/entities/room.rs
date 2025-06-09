use serde::{Deserialize, Serialize};
use crate::models::traits::descriptible::Descriptible;
use std::collections::HashMap;
use crate::models::entities::entity::Entity;
use rand::Rng;

/// Structure utilisée uniquement pour désérialiser le JSON avec la clé "elem"




/// Structure principale utilisée dans le jeu
#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub elem: Entity,
    pub terrain_type: String,
    pub locked: Option<bool>,
    pub items: Vec<u32>,
    pub pnjs: Vec<u32>,
    pub enemies: Vec<u32>,
    pub exits: HashMap<String, usize>,
    pub nord: Option<u32>,
    pub sud: Option<u32>,
    pub est: Option<u32>,
    pub ouest: Option<u32>,
}



impl Descriptible for Room {
    fn get_description(&self) -> String {
        format!("{} - {}", self.name(), self.description())
    }



}

impl Room{
    pub fn id(&self) -> u32 {
        self.elem.id()
    }
    pub fn name(&self) -> &str {
        self.elem.name()
    }
    pub fn description(&self) -> &str {
        self.elem.description()
    }

    pub fn tenter_ouverture(&mut self) -> bool {
        if !self.locked.unwrap_or(true) {
            // Déjà ouverte
            return true;
        }
        println!("🚪 La porte est verrouillée ! Pour l'ouvrir, il faut faire un 421 avec trois dés.");
        let mut des = vec![];
        for _ in 0..3 {
            des.push(rand::thread_rng().gen_range(1..=6));
        }
        println!("🎲 Tu as lancé : {:?}.", des);

        let mut des_tries = des.clone();
        des_tries.sort();
        if des_tries == vec![1, 2, 4] {
            println!("✅ Bravo ! Tu as fait 421, la porte s'ouvre.");
            self.locked = Some(false);
            true
        } else {
            println!("❌ Raté ! Tu ne peux pas entrer.");
            false
        }
    }
}




