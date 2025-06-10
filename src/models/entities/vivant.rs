use serde::{Deserialize, Serialize};
use crate::models::entities::entity::Entity;
use crate::models::entities::inventory::Inventory;
use crate::models::entities::item::Item;
use crate::models::entities::loot_entry::LootEntry;
use crate::models::traits::combattant::Combattant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vivant {
    entity: Entity,
    health: i32,
    strength: i32,
    intelligence: i32,
    defense: i32,
    pub inventory: Inventory,
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

    pub fn inventory(&self) -> &Inventory{
        &self.inventory
    }

    pub fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
    pub fn set_id(&mut self, id: u32) {
        self.entity.set_id(id);
    }

    pub fn set_name(&mut self, name: &str) {
        self.entity.set_name(String::from(name));
    }


}

// Implémentation du trait Combattant pour Vivant
// Permet à l'entité de participer aux combats
impl Combattant for Vivant {
    // Retourne le nom de l'entité
    fn nom(&self) -> &str {
        self.name()
    }

    // Retourne la force de l'entité (minimum 0)
    fn force(&self) -> u32 {
        self.strength().max(0) as u32
    }

    // Retourne les points de vie de l'entité (minimum 0)
    fn sante(&self) -> u32 {
        self.health().max(0) as u32
    }

    // Vérifie si l'entité est en vie
    fn est_vivant(&self) -> bool {
        self.health() > 0
    }

    // Inflige des dégâts à l'entité
    fn infliger_degats(&mut self, degats: u32) {
        self.health = (self.health() - degats as i32).max(0);
    }

    // Retourne les dégâts d'attaque de l'entité
    fn degats_attaque(&self) -> u32 {
        self.strength().max(0) as u32
    }

    // Retourne la protection défensive de l'entité
    fn protection_defense(&self) -> u32 {
        self.defense().max(0) as u32
    }

    fn loot(&self) -> &[LootEntry] {
        &[]
    }
    fn experience_gain(&self) -> i32 {
        0
    }
}