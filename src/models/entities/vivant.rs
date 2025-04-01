use serde::{Deserialize, Serialize};
use crate::models::entities::entity::Entity;
use crate::models::entities::item::Item;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vivant {
    ent: Entity,
    health: u32,
    strength: u32,
    intelligence: u32,
    inventory: Vec<Item>,
}

impl Vivant {

    // GETTERS
    pub fn id(&self) -> u32 {
        self.ent.id()
    }

    pub fn name(&self) -> &str {
        &self.ent.name()
    }

    pub fn description(&self) -> &str {
        &self.ent.description()
    }

    pub fn health(&self) -> u32 {
        self.health
    }

    pub fn strength(&self) -> u32 {
        self.strength
    }

    pub fn intelligence(&self) -> u32 {
        self.intelligence
    }

    // pub fn inventory(&self) -> &Vec<Item> {
    //     &self.inventory
    // }

    pub fn inventory(&mut self) -> &mut Vec<Item> {
        &mut self.inventory
    }
    pub fn set_name(&mut self, name: &str) {
        self.ent.set_name(String::from(name));
    }

    pub fn set_id(&mut self, id: u32) {
        self.ent.set_id(id);
    }

    pub fn set_description(&mut self, desc: &str) {
        self.ent.set_description(String::from(desc));
    }

    pub fn set_health(&mut self, health: u32) {
        self.health = health;
    }

    pub fn set_strength(&mut self, strength: u32) {
        self.strength = strength;
    }

    pub fn set_intelligence(&mut self, intelligence: u32) {
        self.intelligence = intelligence;
    }

    pub fn set_inventory(&mut self, inventory: Vec<Item>) {
        self.inventory = inventory;
    }


}