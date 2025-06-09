// Module de gestion des ennemis du jeu
// Définit la structure Enemy et ses fonctionnalités
// Permet de gérer les ennemis, leurs statistiques et leur loot

use serde::{Deserialize, Serialize};

use crate::models::entities::vivant::Vivant;
use crate::models::entities::inventory_item::InventoryItem;
use crate::models::entities::loot_entry::LootEntry;
use crate::models::traits::combattant::Combattant;

// Structure représentant un ennemi dans le jeu
// Hérite des propriétés de base d'un Vivant et ajoute un système de loot
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Enemy {
    pub vivant: Vivant,                // Propriétés de base (santé, force, etc.)
    loot: Vec<LootEntry>,         // Liste des objets que l'ennemi peut lâcher
}

impl Enemy {
    // Crée un nouvel ennemi avec ses statistiques et son loot
    pub fn new(vivant: Vivant, loot: Vec<LootEntry>) -> Self {
        Self {
            vivant,
            loot,
        }
    }

    // Retourne l'identifiant unique de l'ennemi
    pub fn id(&self) -> u32 {
        self.vivant.id()
    }

    // Retourne le nom de l'ennemi
    pub fn name(&self) -> &str {
        self.vivant.name()
    }

    // Retourne les points de vie actuels de l'ennemi
    pub fn health(&self) -> i32 {
        self.vivant.health()
    }

    // Retourne la force de l'ennemi
    pub fn strength(&self) -> i32 {
        self.vivant.strength()
    }

    // Retourne l'intelligence de l'ennemi
    pub fn intelligence(&self) -> i32 {
        self.vivant.intelligence()
    }

    // Modifie les points de vie de l'ennemi
    pub fn set_health(&mut self, health: i32) {
        self.vivant.set_health(health);
    }

    // Modifie la force de l'ennemi
    pub fn set_strength(&mut self, strength: i32) {
        self.vivant.set_strength(strength);
    }

    // Modifie l'intelligence de l'ennemi
    pub fn set_intelligence(&mut self, intelligence: i32) {
        self.vivant.set_intelligence(intelligence);
    }

    // Vérifie si l'ennemi est en vie
    pub fn is_alive(&self) -> bool {
        self.vivant.health() > 0
    }

    // Retourne la liste des objets que l'ennemi peut lâcher
    pub fn drop_loot(&self) -> Vec<InventoryItem> {
        LootEntry::generer_depuis_table(&self.loot)
    }

    // Retourne la défense de l'ennemi
    pub fn defense(&self) -> i32 {
        self.vivant.defense()
    }
}

// Implémentation du trait Combattant pour Enemy
// Permet à l'ennemi de participer aux combats
impl Combattant for Enemy {
    // Retourne le nom de l'ennemi
    fn nom(&self) -> &str {
        self.name()
    }

    // Retourne la force de l'ennemi (minimum 0)
    fn force(&self) -> u32 {
        self.strength().max(0) as u32
    }

    // Retourne les points de vie de l'ennemi (minimum 0)
    fn sante(&self) -> u32 {
        self.health().max(0) as u32
    }

    // Vérifie si l'ennemi est en vie
    fn est_vivant(&self) -> bool {
        self.health() > 0
    }

    // Inflige des dégâts à l'ennemi
    fn infliger_degats(&mut self, degats: u32) {
        self.set_health((self.health() - degats as i32).max(0));
    }

    // Retourne les dégâts d'attaque de l'ennemi
    fn degats_attaque(&self) -> u32 {
        self.strength().max(0) as u32
    }

    // Retourne la protection défensive de l'ennemi
    fn protection_defense(&self) -> u32 {
        self.defense().max(0) as u32
    }

    fn loot(&self) -> &[LootEntry] {
        &self.loot
    }
    fn experience_gain(&self) -> i32 {
        //self.vivant.strength + self.vivant.defense + self.vivant.intelligence
        self.vivant.experience
    }
}


