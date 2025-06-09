use serde::{Deserialize, Serialize};
use crate::models::entities::entity::Entity;
use crate::models::entities::inventory::Inventory;
use crate::models::entities::item::Item;
use crate::models::traits::combattant::Combattant;

// Structure qui représente une entité vivante dans le jeu
// Contient les stats de base et l'inventaire d'un personnage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vivant {
    entity: Entity,        // Informations de base (nom, description, etc.)
    health: i32,           // Points de vie actuels
    strength: i32,         // Force pour les dégâts d'attaque
    intelligence: i32,     // Intelligence pour les sorts et interactions
    defense: i32,          // Défense pour réduire les dégâts reçus
    pub inventory: Inventory, // Inventaire du personnage
}

impl Vivant {
    // Getters pour accéder aux attributs de base
    pub fn id(&self) -> u32 { self.entity.id() }
    pub fn name(&self) -> &str { self.entity.name() }
    pub fn description(&self) -> &str { self.entity.description() }

    // Getters pour les statistiques
    pub fn health(&self) -> i32 { self.health }
    pub fn intelligence(&self) -> i32 { self.intelligence }
    pub fn strength(&self) -> i32 { self.strength }

    // Setters pour modifier les statistiques
    pub fn set_health(&mut self, health: i32) {
        self.health = health;
    }
    pub fn set_strength(&mut self, strength: i32) {
        self.strength = strength;
    }
    pub fn set_intelligence(&mut self, intelligence: i32) {
        self.intelligence = intelligence;
    }

    // Getter pour la défense
    pub fn defense(&self) -> i32 {
        self.defense
    }

    // Getters pour l'inventaire (mutable et immutable)
    pub fn inventory(&self) -> &Inventory{
        &self.inventory
    }
    pub fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    // Setters pour les attributs de base
    pub fn set_id(&mut self, id: u32) {
        self.entity.set_id(id);
    }
    pub fn set_name(&mut self, name: &str) {
        self.entity.set_name(String::from(name));
    }
}

// Implémentation du trait Combattant pour les entités vivantes
impl Combattant for Vivant {
    // Retourne le nom du combattant
    fn nom(&self) -> &str {
        self.name()
    }

    // Calcule la force d'attaque (minimum 0)
    fn force(&self) -> u32 {
        self.strength().max(0) as u32
    }

    // Calcule les points de vie actuels (minimum 0)
    fn sante(&self) -> u32 {
        self.health().max(0) as u32
    }

    // Vérifie si l'entité est en vie
    fn est_vivant(&self) -> bool {
        self.health() > 0
    }

    // Applique des dégâts à l'entité
    fn infliger_degats(&mut self, degats: u32) {
        self.health = (self.health() - degats as i32).max(0);
    }

    // Calcule les dégâts d'attaque
    fn degats_attaque(&self) -> u32 {
        self.strength().max(0) as u32
    }

    // Calcule la protection défensive
    fn protection_defense(&self) -> u32 {
        self.defense().max(0) as u32
    }
}
