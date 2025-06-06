use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::entities::character::Character;
use crate::models::entities::entity::Entity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quete {
    entity: Entity,
    pub dialogue_id: u32,
    pub objectif_type: String,
    pub objectif: Objectif,
    pub recompense_items: Vec<u32>,
    pub recompense_argent: i32,
    pub experience: i32,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Objectif {
    pub tuer: Tuer,
    pub collecter: Collecter,
    pub visiter: Visiter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tuer {
    pub ennemi_id: u32,
    pub target: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collecter {
    pub item_id: u32,
    pub target: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Visiter {
    room_id: u32,
    visited: bool,
}

impl Quete {

    pub fn id(&self) -> u32 {
        self.entity.id()
    }

    pub fn name(&self) -> &str {
        self.entity.name()
    }

    pub fn item_id(&self) -> u32 {
        self.objectif.collecter.item_id
    }

    pub fn inc_item_count(&mut self) {
        self.objectif.collecter.count += 1;
    }

    /// Vérifie si le nombre requis d'objets collectés a été atteint.
    /// Retourne `true` si le joueur a collecté le nombre cible d'objets, sinon `false`.
    pub fn is_item_count_reached(&self) -> bool {
        // Compare le nombre actuel d'objets collectés avec le nombre cible requis.
        if self.objectif.collecter.count == self.objectif.collecter.target {
            return true // La condition est remplie, l'objectif de collecte est atteint.
        }
        false // La condition n'est pas remplie, l'objectif n'est pas encore atteint.
    }


    pub fn ennemi_id(&self) -> u32 {
        self.objectif.tuer.ennemi_id
    }

    pub fn inc_ennemi_count(&mut self) {
        self.objectif.tuer.count += 1;
    }

    pub fn is_ennemi_count_reached(&self) -> bool {
        if self.objectif.tuer.count == self.objectif.tuer.target {
            return true
        }
        false
    }


}