use serde::{Deserialize, Serialize};

use crate::models::entities::vivant::Vivant;
use crate::models::entities::inventory_item::InventoryItem;
use crate::models::traits::combattant::Combattant;

// Structure qui représente un ennemi dans le jeu
// Hérite des attributs de base d'une entité vivante et ajoute un système de loot
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Enemy {
    pub vivant: Vivant,           // Attributs de base d'une entité vivante (stats, etc.)
    loot: Vec<InventoryItem>,     // Liste des objets que l'ennemi peut laisser tomber
}

impl Enemy {
    // Crée un nouvel ennemi avec ses attributs de base et son loot
    pub fn new(vivant: Vivant, loot: Vec<InventoryItem>) -> Self {
        Self {
            vivant,
            loot,
        }
    }

    // Getters pour les attributs de base
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

    // Setters pour modifier les statistiques
    pub fn set_health(&mut self, health: i32) {
        self.vivant.set_health(health);
    }

    pub fn set_strength(&mut self, strength: i32) {
        self.vivant.set_strength(strength);
    }

    pub fn set_intelligence(&mut self, intelligence: i32) {
        self.vivant.set_intelligence(intelligence);
    }

    // Vérifie si l'ennemi est en vie
    pub fn is_alive(&self) -> bool {
        self.vivant.health() > 0
    }

    // Retourne la liste des objets que l'ennemi peut laisser tomber
    pub fn drop_loot(&self) -> Vec<InventoryItem> {
        self.loot.clone()
    }

    // Getter pour la défense
    pub fn defense(&self) -> i32 {
        self.vivant.defense()
    }
}

// Implémentation du trait Combattant pour les combats
impl Combattant for Enemy {
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

    // Vérifie si l'ennemi est en vie
    fn est_vivant(&self) -> bool {
        self.health() > 0
    }

    // Applique des dégâts à l'ennemi
    fn infliger_degats(&mut self, degats: u32) {
        self.set_health((self.health() - degats as i32).max(0));
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