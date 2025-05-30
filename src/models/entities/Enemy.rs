use serde::{Deserialize, Serialize};

use crate::models::entities::vivant::Vivant;
use crate::models::entities::inventory_item::InventoryItem;
use crate::models::traits::combattant::Combattant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Enemy {
    pub vivant: Vivant,
    loot: Vec<InventoryItem>,
}


impl Enemy {
    pub fn new(vivant: Vivant, loot: Vec<InventoryItem>) -> Self{
        Self {
            vivant,
            loot,
        }
    }


    pub fn id(&self) -> u32 {
        self.vivant.id()
    }

    pub fn name(&self) -> &str {
        self.vivant.name()
    }

    pub fn health(&self) -> i32 {
        self.vivant.health()
    }

    pub fn strength(&self) -> i32 {
        self.vivant.strength()
    }

    pub fn intelligence(&self) -> i32 {
        self.vivant.intelligence()
    }

    pub fn set_health(&mut self, health: i32) {
        self.vivant.set_health(health);
    }

    pub fn set_strength(&mut self, strength: i32) {
        self.vivant.set_strength(strength);
    }


    pub fn set_intelligence(&mut self, intelligence: i32) {
        self.vivant.set_intelligence(intelligence);
    }

    pub fn is_alive(&self) -> bool{

        self.vivant.health() > 0
    }

    pub fn drop_loot(&self) -> Vec<InventoryItem> {
        // Implémente ta logique pour récupérer les objets
        self.loot.clone()
    }

    pub fn defense(&self) -> i32 {
        self.vivant.defense()
    }


}


impl Combattant for Enemy {

    fn nom(&self) -> &str {
        self.name()
    }

    fn force(&self) -> u32 {
        self.strength().max(0) as u32
    }

    fn sante(&self) -> u32 {
        self.health().max(0) as u32
    }

    fn est_vivant(&self) -> bool {
        self.health() > 0
    }
    fn infliger_degats(&mut self, degats: u32) {
        self.set_health( (self.health() - degats as i32).max(0) );
    }

    fn degats_attaque(&self) -> u32 {
        self.strength().max(0) as u32
    }

    fn protection_defense(&self) -> u32 {
        self.defense().max(0) as u32
    }

}


/*impl Combattant for Enemy {
    fn nom(&self) -> &str { self.vivant.nom() }
    fn force(&self) -> u32 { self.vivant.force() }
    fn sante(&self) -> u32 { self.vivant.sante() }
    fn est_vivant(&self) -> bool { self.vivant.est_vivant() }

    fn infliger_degats(&mut self, degats: u32) {
        self.vivant.infliger_degats(degats);
    }

    fn degats_attaque(&self) -> u32 {
        self.vivant.degats_attaque()
    }

    fn protection_defense(&self) -> u32 {
        self.vivant.protection_defense()
    }
}*/