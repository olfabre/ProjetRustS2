use serde::{Deserialize, Serialize};
use crate::models::traits::descriptible::Descriptible;
use std::collections::HashMap;
use crate::models::entities::entity::Entity;
use rand::Rng;

/// Structure utilisée uniquement pour désérialiser le JSON avec la clé "elem"

/// Structure qui représente une salle dans le jeu
/// Contient les informations sur l'environnement, les objets, PNJs et ennemis présents
#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub elem: Entity,                    // Informations de base (nom, description, etc.)
    pub terrain_type: String,            // Type de terrain de la salle
    pub locked: Option<bool>,            // Indique si la salle est verrouillée
    pub items: Vec<u32>,                 // Liste des IDs des objets présents dans la salle
    pub pnjs: Vec<u32>,                  // Liste des IDs des PNJs présents dans la salle
    pub enemies: Vec<u32>,               // Liste des IDs des ennemis présents dans la salle
    pub exits: HashMap<String, usize>,   // Sorties disponibles et leurs destinations
    pub nord: Option<u32>,               // ID de la salle au nord (optionnel)
    pub sud: Option<u32>,                // ID de la salle au sud (optionnel)
    pub est: Option<u32>,                // ID de la salle à l'est (optionnel)
    pub ouest: Option<u32>,              // ID de la salle à l'ouest (optionnel)
}

impl Descriptible for Room {
    fn get_description(&self) -> String {
        format!("{} - {}", self.name(), self.description())
    }
}

impl Room {
    // Getters pour les attributs de base
    pub fn id(&self) -> u32 {
        self.elem.id()
    }

    pub fn name(&self) -> &str {
        self.elem.name()
    }

    pub fn description(&self) -> &str {
        self.elem.description()
    }

    // Tente d'ouvrir une salle verrouillée en lançant trois dés
    // Retourne true si la salle est déverrouillée avec succès
    pub fn tenter_ouverture(&mut self) -> bool {
        // Si la salle n'est pas verrouillée, on peut entrer
        if !self.locked.unwrap_or(true) {
            return true;
        }

        // Demande au joueur de lancer les dés pour faire un 421
        println!("🚪 La porte est verrouillée ! Pour l'ouvrir, il faut faire un 421 avec trois dés.");
        let mut des = vec![];
        for _ in 0..3 {
            des.push(rand::thread_rng().gen_range(1..=6));
        }
        println!("🎲 Tu as lancé : {:?}.", des);

        // Vérifie si le joueur a fait un 421
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




