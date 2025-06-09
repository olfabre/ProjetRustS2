// Module de gestion des entités vivantes du jeu
// Définit la structure Vivant qui sert de base pour les personnages et ennemis
// Gère les statistiques de base et l'inventaire des entités vivantes

use serde::{Deserialize, Serialize};
use crate::models::entities::entity::Entity;
use crate::models::entities::inventory::Inventory;
use crate::models::entities::item::Item;
use crate::models::traits::combattant::Combattant;

// Structure représentant une entité vivante dans le jeu
// Contient les statistiques de base et l'inventaire
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vivant {
    entity: Entity,           // Informations de base (nom, description)
    health: i32,             // Points de vie
    strength: i32,           // Force (dégâts d'attaque)
    intelligence: i32,       // Intelligence (pour les sorts)
    defense: i32,            // Défense (réduction des dégâts)
    pub inventory: Inventory, // Inventaire de l'entité
}

impl Vivant {
    // Retourne l'identifiant unique de l'entité
    pub fn id(&self) -> u32 { self.entity.id() }

    // Retourne le nom de l'entité
    pub fn name(&self) -> &str { self.entity.name() }

    // Retourne la description de l'entité
    pub fn description(&self) -> &str { self.entity.description() }

    // Retourne les points de vie actuels
    pub fn health(&self) -> i32 { self.health }

    // Retourne l'intelligence de l'entité
    pub fn intelligence(&self) -> i32 { self.intelligence }

    // Retourne la force de l'entité
    pub fn strength(&self) -> i32 { self.strength }

    // Modifie les points de vie de l'entité
    pub fn set_health(&mut self, health: i32) {
        self.health = health;
    }

    // Modifie la force de l'entité
    pub fn set_strength(&mut self, strength: i32) {
        self.strength = strength;
    }

    // Modifie l'intelligence de l'entité
    pub fn set_intelligence(&mut self, intelligence: i32) {
        self.intelligence = intelligence;
    }

    // Retourne la défense de l'entité
    pub fn defense(&self) -> i32 {
        self.defense
    }

    // Retourne une référence à l'inventaire
    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    // Retourne une référence mutable à l'inventaire
    pub fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    // Modifie l'identifiant de l'entité
    pub fn set_id(&mut self, id: u32) {
        self.entity.set_id(id);
    }

    // Modifie le nom de l'entité
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
}
