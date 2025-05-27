use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::entities::character::Character;
use crate::models::entities::entity::Entity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quete {
    entity: Entity,
    // pub statu: String, // disponible, accepté, terminé, rendu
    pub dialog_rendu_id: u32, // reference to dialogue, can belong to a different pnj other than the quest giver
    pub objectif_type: String,
    pub objectif: Objectif,
    pub recompense_items: Vec<u32>,
    pub recompense_argent: i32,
    pub experience: i32,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Objectif {
    tuer: Tuer,
    pub(crate) collecter: Collecter,
    visiter: Visiter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tuer {
    ennemi_id: u32,
    target: u32,
    count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collecter {
    item_id: u32,
    target: u32,
    count: u32,
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

    pub fn is_item_count_reached(&self) -> bool {
        if self.objectif.collecter.count == self.objectif.collecter.target {
            return true
        }
        false
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