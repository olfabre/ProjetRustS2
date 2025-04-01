use serde::{Deserialize, Serialize};
use crate::models::traits::Descriptible;
use std::collections::HashMap;

/// Structure utilisée uniquement pour désérialiser le JSON avec la clé "elem"
#[derive(Debug, Deserialize)]
pub struct RoomWrapper {
    pub elem: RoomElem,
    pub terrain_type: String,
    pub locked: Option<bool>,
    pub items: Vec<u32>,
    pub pnjs: Vec<u32>,
    pub enemies: Vec<u32>,
    pub exits: HashMap<String, usize>,
}

#[derive(Debug, Deserialize)]
pub struct RoomElem {
    pub id: usize,
    pub name: String,
    pub description: String,
}

/// Structure principale utilisée dans le jeu
#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub terrain_type: String,
    pub sombre: Option<bool>,
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

impl From<RoomWrapper> for Room {
    fn from(wrapper: RoomWrapper) -> Self {
        Room {
            id: wrapper.elem.id,
            name: wrapper.elem.name,
            description: wrapper.elem.description,
            terrain_type: wrapper.terrain_type,
            sombre: None,
            locked: wrapper.locked,
            items: wrapper.items,
            pnjs: wrapper.pnjs,
            enemies: wrapper.enemies,
            exits: wrapper.exits,
            north: None,
            south: None,
            east: None,
            west: None,
        }
    }
}

impl Descriptible for Room {
    fn get_description(&self) -> String {
        format!("{} - {}", self.name, self.description)
    }
}
