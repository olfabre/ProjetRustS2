use serde::{Deserialize, Serialize};
use crate::models::traits::Descriptible;
use std::collections::HashMap;
use crate::models::entities::entity::Entity;

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
    pub north: Option<u32>,
    pub south: Option<u32>,
    pub east: Option<u32>,
    pub west: Option<u32>,
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

}
