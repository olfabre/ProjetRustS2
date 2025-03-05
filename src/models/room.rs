use serde::{Deserialize, Serialize};
use crate::models::traits::Descriptible;


#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub terrain_type: String, // Forêt, Donjon, Village, etc.
    pub items: Vec<String>,   // Liste des objets trouvables ici
    pub npcs: Vec<String>,    // Liste des PNJ présents


    pub north: Option<u32>,
    pub south: Option<u32>,
    pub east: Option<u32>,
    pub west: Option<u32>,
}

impl Descriptible for Room{
    fn get_description(&self) -> String {
        format!("{} - {}", self.name, self.description)
    }
}