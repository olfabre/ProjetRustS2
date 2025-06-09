// Module de gestion des objets du jeu
// Définit la structure Item et ses fonctionnalités
// Permet de gérer les objets utilisables, équipables et leurs effets

use serde::{Deserialize, Serialize};
use crate::models::entities::entity::Entity;
use crate::models::traits::descriptible::Descriptible;
use crate::models::traits::interactible::Interactable;

// Structure représentant un objet dans le jeu
// Hérite des propriétés de base d'une entité et ajoute des caractéristiques spécifiques
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item { 
    entity: Entity,        // Informations de base de l'objet (nom, description, etc.)
    pub effect: String,    // Effet de l'objet lorsqu'il est utilisé
    pub usable: bool,      // Indique si l'objet peut être utilisé
    pub weight: f32,       // Poids de l'objet (pour la gestion de l'inventaire)
    pub value: i32,        // Valeur monétaire de l'objet
    pub stackable: bool,   // Indique si l'objet peut être empilé
    pub equippable: bool,  // Indique si l'objet peut être équipé
}

// Implémentation du trait Descriptible pour Item
// Permet d'obtenir une description complète de l'objet
impl Descriptible for Item {
    fn get_description(&self) -> String {
        format!("{} : {}", self.entity.name(), self.entity.description())
    }
}

// Implémentation du trait Interactable pour Item
// Gère l'interaction avec l'objet (utilisation)
impl Interactable for Item {
    fn interact(&self) {
        println!("Vous utilisez l'objet : {}", self.entity.name());
    }
}

impl Item {
    // Retourne l'identifiant unique de l'objet
    pub fn id(&self) -> u32 {
        self.entity.id()
    }

    // Retourne le nom de l'objet
    pub fn name(&self) -> &str {
        &self.entity.name()
    }

    // Retourne la description de l'objet
    pub fn description(&self) -> &str {
        self.entity.description()
    }

    // Vérifie si l'objet peut être utilisé
    pub fn is_usable(&self) -> bool {
        self.usable
    }

    // Retourne l'effet de l'objet
    pub fn effect(&self) -> &str {
        &self.effect
    }
}