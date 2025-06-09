// Module de gestion des objets du jeu
// Définit la structure et les comportements des items utilisables par le joueur

use serde::{Deserialize, Serialize};
use crate::models::entities::entity::Entity;
use crate::models::traits::descriptible::Descriptible;
use crate::models::traits::interactible::Interactable;

// Structure représentant un objet du jeu
// Hérite des attributs de base d'une entité et ajoute des propriétés spécifiques aux objets
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item { 
    entity: Entity,      // Informations de base de l'objet
    pub effect: String,  // Effet de l'objet lorsqu'il est utilisé
    pub usable: bool,    // Indique si l'objet peut être utilisé
    pub weight: f32,     // Poids de l'objet (pour la gestion de l'inventaire)
    pub value: i32,      // Valeur monétaire de l'objet
    pub stackable: bool, // Indique si l'objet peut être empilé
    pub equippable: bool,// Indique si l'objet peut être équipé
}

// Implémentation du trait Descriptible pour les objets
// Permet d'obtenir une description formatée de l'objet
impl Descriptible for Item {
    fn get_description(&self) -> String {
        format!("{} : {}", self.entity.name(), self.entity.description())
    }
}

// Implémentation du trait Interactable pour les objets
// Définit le comportement lors de l'utilisation de l'objet
impl Interactable for Item {
    fn interact(&self) {
        println!("Vous utilisez l'objet : {}", self.entity.name());
    }
}

impl Item {
    // Getters pour accéder aux attributs de l'objet
    pub fn id(&self) -> u32 {
        self.entity.id()
    }
    pub fn name(&self) -> &str {
        &self.entity.name()
    }

    pub fn description(&self) -> &str {
        self.entity.description()
    }

    pub fn is_usable(&self) -> bool {
        self.usable
    }

    pub fn effect(&self) -> &str {
        &self.effect
    }
}