use serde::{Deserialize, Serialize};
use crate::models::traits::Descriptible;
use std::collections::HashMap;





#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub terrain_type: String, // Forêt, Donjon, Village, etc.
    pub sombre: Option<bool>,
    pub locked: Option<bool>,  // Ajout du champ locked
    pub items: Vec<u32>,   // Liste des objets trouvables ici
    pub pnjs: Vec<u32>, // Liste des PNJ présents
    pub enemies: Vec<u32>, // ennemie présent dans la salle

    pub exits: HashMap<String, usize>, // Clé = "north", "south", etc. ; Valeur = index de la salle
}

impl Descriptible for Room{
    fn get_description(&self) -> String {
        format!("{} - {}", self.name, self.description)
    }
}