// Module de gestion des salles du jeu
// Définit la structure Room et ses fonctionnalités
// Permet de gérer les salles, leurs contenus et leurs connexions

use serde::{Deserialize, Serialize};
use crate::models::traits::descriptible::Descriptible;
use std::collections::HashMap;
use crate::models::entities::entity::Entity;
use rand::Rng;

/// Structure utilisée uniquement pour désérialiser le JSON avec la clé "elem"

// Structure principale représentant une salle dans le jeu
// Contient toutes les informations sur la salle et ses connexions
#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub elem: Entity,                // Informations de base de la salle
    pub terrain_type: String,        // Type de terrain (forêt, donjon, etc.)
    pub locked: Option<bool>,        // État de verrouillage de la salle
    pub items: Vec<u32>,            // Liste des objets présents dans la salle
    pub pnjs: Vec<u32>,             // Liste des PNJ présents dans la salle
    pub enemies: Vec<u32>,          // Liste des ennemis présents dans la salle
    pub exits: HashMap<String, usize>, // Sorties disponibles et leurs destinations
    pub nord: Option<u32>,          // ID de la salle au nord (optionnel)
    pub sud: Option<u32>,           // ID de la salle au sud (optionnel)
    pub est: Option<u32>,           // ID de la salle à l'est (optionnel)
    pub ouest: Option<u32>,         // ID de la salle à l'ouest (optionnel)
}

// Implémentation du trait Descriptible pour Room
// Permet d'obtenir une description complète de la salle
impl Descriptible for Room {
    fn get_description(&self) -> String {
        format!("{} - {}", self.name(), self.description())
    }
}

impl Room {
    // Retourne l'identifiant unique de la salle
    pub fn id(&self) -> u32 {
        self.elem.id()
    }

    // Retourne le nom de la salle
    pub fn name(&self) -> &str {
        self.elem.name()
    }

    // Retourne la description de la salle
    pub fn description(&self) -> &str {
        self.elem.description()
    }

    // Tente d'ouvrir une salle verrouillée
    // Utilise un mini-jeu de dés (421) pour déverrouiller
    // Retourne true si la salle est déverrouillée, false sinon
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




