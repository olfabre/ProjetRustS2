
use serde::{Deserialize, Serialize};
use crate::models::entities::entity::Entity;
use crate::models::traits::Descriptible;
use crate::models::traits::Interactable;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item { 

    entity: Entity,
    item_type: String,
    pub effect: String,
    pub usable: bool,
    pub weight: f32,
    pub value: u32,
    pub stackable: bool,
    pub equippable: bool,
}

impl Descriptible for Item {
    fn get_description(&self) -> String {
        format!("🧱 {} ({}) : {}", self.entity.name(), self.item_type, self.entity.description())
    }
}

impl Interactable for Item {
    fn interact(&self) {
        println!("Vous utilisez l'objet : {}", self.entity.name());
    }
}

impl Item {

    pub fn id(&self) -> u32 {
        self.entity.id()
    }
    pub fn name(&self) -> &str {
        &self.entity.name()
    }

    pub fn description(&self) -> &str {
        self.entity.description()
    }

    pub fn usable(&self) -> bool {
        self.usable
    }

    pub fn effect(&self) -> &str {
        &self.effect
    }

    pub fn item_type(&self) -> &str {
        &self.item_type
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn stackable(&self) -> bool {
        self.stackable
    }

    pub fn equippable(&self) -> bool {
        self.equippable
    }

}