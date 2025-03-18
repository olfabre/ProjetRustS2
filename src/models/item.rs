use crate::models::traits::Descriptible;
use crate::models::traits::Interactable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub effect: Option<String>,
    pub usable: bool,
}

impl Descriptible for Item {
    fn get_description(&self) -> String {
        format!("{} : {}", self.name, self.description)
    }
}

impl Interactable for Item {
    fn interact(&self) {
        println!("Vous utilisez l'objet : {}", self.name);
    }
}
