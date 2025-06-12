
use serde::{Deserialize, Serialize};
use crate::models::entities::entity::Entity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quete {
    entity: Entity,
    pub dialogue_rendu_id: u32,
    pub objectif: Objectif,
    pub recompense_items: Vec<u32>,
    pub recompense_argent: i32,
    pub experience: i32,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")] // Helpful for distinguishing types
pub enum Objectif {
    Tuer(Tuer),
    Collecter(Collecter),
    Visiter(Visiter),
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


impl Objectif {

    pub fn target_id(&self) -> u32 {
        match self {
            Objectif::Collecter(obj) => obj.item_id,
            Objectif::Tuer(obj) => obj.ennemi_id,
            Objectif::Visiter(obj) => obj.room_id,

        }
    }

    pub fn increment_count(&mut self) {
        match self {
            Objectif::Tuer(ref mut obj) => obj.count += 1,
            Objectif::Collecter(ref mut obj) => obj.count += 1,
            Objectif::Visiter(ref mut obj) => obj.visited = true,
        }
    }

    pub fn target(&self) -> u32 {
        match self {
            Objectif::Tuer(obj) => obj.target,
            Objectif::Collecter(obj) => obj.target,
            Objectif::Visiter(obj) => if obj.visited { 1 } else { 0 },
        }
    }

    /// Vérifie si le nombre requis d'objets collectés a été atteint.
    /// Retourne `true` si le joueur a collecté le nombre cible d'objets, sinon `false`.
    pub fn is_complete(&self) -> bool {
        match self {
            Objectif::Tuer(obj) => obj.count == obj.target,
            Objectif::Collecter(obj) => obj.count == obj.target,
            Objectif::Visiter(obj) => obj.visited,
        }
    }

    pub fn type_str(&self) -> &'static str {
        match self {
            Objectif::Tuer(_) => "tuer",
            Objectif::Collecter(_) => "collecter",
            Objectif::Visiter(_) => "visiter",
        }
    }



}


impl Quete {

    pub fn id(&self) -> u32 {
        self.entity.id()
    }

    pub fn name(&self) -> &str {
        self.entity.name()
    }

    pub fn target_id(&self) -> u32 {
        self.objectif.target_id()
    }

    pub fn target(&self) -> u32 {
        self.objectif.target()
    }

    pub fn increment_count(&mut self) {
        self.objectif.increment_count()
    }

    pub fn is_complete(&self) -> bool {
        self.objectif.is_complete()
    }

    pub fn get_type(&self) -> &str {
        self.objectif.type_str()
    }

}