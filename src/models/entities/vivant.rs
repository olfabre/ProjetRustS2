use serde::{Deserialize, Serialize};
use crate::models::entities::entity::Entity;
use crate::models::entities::item::Item;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vivant {
    entity: Entity,
    health: i32,
    strength: i32,
    intelligence: i32,
    defense: i32,
    inventory: Vec<Item>,
}

impl Vivant {

    pub fn id(&self) -> u32 { self.entity.id() }

    pub fn name(&self) -> &str { self.entity.name() }

    pub fn description(&self) -> &str { self.entity.description() }

    pub fn health(&self) -> i32 { self.health }

    pub fn intelligence(&self) -> i32 { self.intelligence }

    pub fn strength(&self) -> i32 { self.strength }

    pub fn set_health(&mut self, health: i32) {
        self.health = health;
    }

    pub fn set_strength(&mut self, strength: i32) {
        self.strength = strength;
    }

    pub fn set_intelligence(&mut self, intelligence: i32) {
        self.intelligence = intelligence;
    }

    pub fn defense(&self) -> i32 {
        self.defense
    }

    pub fn inventory(self) -> Vec<Item> {
        self.inventory
    }

    pub fn inventory_mut(&mut self) -> &mut Vec<Item> {
        &mut self.inventory
    }

    pub fn set_name(&mut self, name: &str) {
        self.entity.set_name(String::from(name));
    }
}