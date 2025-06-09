use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::entities::character::Character;
use crate::models::entities::entity::Entity;

// Structure qui représente une quête dans le jeu
// Définit les objectifs à accomplir et les récompenses associées
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quete {
<<<<<<< HEAD
    entity: Entity,                // Informations de base (nom, description, etc.)
    // pub statu: String, // disponible, accepté, terminé, rendu
    pub dialog_rendu_id: u32,      // ID du dialogue à afficher lors de la remise de la quête
    pub objectif_type: String,     // Type d'objectif (tuer, collecter, visiter)
    pub objectif: Objectif,        // Détails de l'objectif à accomplir
    pub recompense_items: Vec<u32>, // Liste des IDs des objets à donner en récompense
    pub recompense_argent: i32,    // Montant d'argent à donner en récompense
    pub experience: i32,           // Points d'expérience à donner en récompense
=======
    entity: Entity,
    pub dialog_rendu_id: u32,
    pub objectif_type: String,
    pub objectif: Objectif,
    pub recompense_items: Vec<u32>,
    pub recompense_argent: i32,
    pub experience: i32,

>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
}

// Structure qui regroupe les différents types d'objectifs possibles
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Objectif {
    pub tuer: Tuer,           // Objectif de type "tuer des ennemis"
    pub collecter: Collecter,  // Objectif de type "collecter des objets"
    pub visiter: Visiter,     // Objectif de type "visiter une salle"
}

// Structure qui définit un objectif de type "tuer des ennemis"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tuer {
    pub ennemi_id: u32,    // ID du type d'ennemi à tuer
    pub target: u32,       // Nombre d'ennemis à tuer
    pub count: u32,        // Nombre d'ennemis déjà tués
}

// Structure qui définit un objectif de type "collecter des objets"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collecter {
    pub item_id: u32,      // ID du type d'objet à collecter
    pub target: u32,       // Nombre d'objets à collecter
    pub count: u32,        // Nombre d'objets déjà collectés
}

// Structure qui définit un objectif de type "visiter une salle"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Visiter {
    room_id: u32,          // ID de la salle à visiter
    visited: bool,         // Indique si la salle a été visitée
}

impl Quete {
<<<<<<< HEAD
    // Getters pour les attributs de base
=======

>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
    pub fn id(&self) -> u32 {
        self.entity.id()
    }

    pub fn name(&self) -> &str {
        self.entity.name()
    }

    // Getters et méthodes pour l'objectif de collecte
    pub fn item_id(&self) -> u32 {
        self.objectif.collecter.item_id
    }

    pub fn inc_item_count(&mut self) {
        self.objectif.collecter.count += 1;
    }

    /// Vérifie si le nombre requis d'objets collectés a été atteint.
    /// Retourne `true` si le joueur a collecté le nombre cible d'objets, sinon `false`.
    pub fn is_item_count_reached(&self) -> bool {
        // Compare le nombre actuel d'objets collectés avec le nombre cible requis.
        if self.objectif.collecter.count == self.objectif.collecter.target {
            return true // La condition est remplie, l'objectif de collecte est atteint.
        }
        false // La condition n'est pas remplie, l'objectif n'est pas encore atteint.
    }

<<<<<<< HEAD
    // Getters et méthodes pour l'objectif de combat
=======

>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
    pub fn ennemi_id(&self) -> u32 {
        self.objectif.tuer.ennemi_id
    }

    pub fn inc_ennemi_count(&mut self) {
        self.objectif.tuer.count += 1;
    }

    pub fn is_ennemi_count_reached(&self) -> bool {
        if self.objectif.tuer.count == self.objectif.tuer.target {
            return true
        }
        false
    }
}