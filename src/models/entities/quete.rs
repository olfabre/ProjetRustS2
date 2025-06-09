// Module de gestion des quêtes du jeu
// Définit les structures pour les quêtes et leurs objectifs
// Permet de gérer la progression et les récompenses des quêtes

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::entities::character::Character;
use crate::models::entities::entity::Entity;

// Structure principale représentant une quête
// Contient les informations de base, les objectifs et les récompenses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quete {
    entity: Entity,                    // Informations de base de la quête
    // pub statu: String, // disponible, accepté, terminé, rendu
    pub dialog_rendu_id: u32,         // ID du dialogue pour rendre la quête
    pub objectif_type: String,        // Type d'objectif (tuer, collecter, visiter)
    pub objectif: Objectif,           // Détails des objectifs à accomplir
    pub recompense_items: Vec<u32>,   // Liste des objets en récompense
    pub recompense_argent: i32,       // Montant d'argent en récompense
    pub experience: i32,              // Points d'expérience en récompense
}

// Structure contenant tous les types d'objectifs possibles
// Une quête peut avoir plusieurs types d'objectifs simultanément
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Objectif {
    pub tuer: Tuer,           // Objectif de combat
    pub collecter: Collecter,  // Objectif de collecte
    pub visiter: Visiter,     // Objectif d'exploration
}

// Structure pour les objectifs de type "tuer des ennemis"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tuer {
    pub ennemi_id: u32,  // Type d'ennemi à vaincre
    pub target: u32,     // Nombre d'ennemis à vaincre
    pub count: u32,      // Nombre d'ennemis déjà vaincus
}

// Structure pour les objectifs de type "collecter des objets"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collecter {
    pub item_id: u32,  // Type d'objet à collecter
    pub target: u32,   // Nombre d'objets à collecter
    pub count: u32,    // Nombre d'objets déjà collectés
}

// Structure pour les objectifs de type "visiter un lieu"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Visiter {
    room_id: u32,    // ID de la salle à visiter
    visited: bool,   // Indique si la salle a été visitée
}

impl Quete {
    // Retourne l'identifiant unique de la quête
    pub fn id(&self) -> u32 {
        self.entity.id()
    }

    // Retourne le nom de la quête
    pub fn name(&self) -> &str {
        self.entity.name()
    }

    // Retourne l'ID de l'objet à collecter
    pub fn item_id(&self) -> u32 {
        self.objectif.collecter.item_id
    }

    // Incrémente le compteur d'objets collectés
    pub fn inc_item_count(&mut self) {
        self.objectif.collecter.count += 1;
    }

    // Vérifie si l'objectif de collecte est atteint
    pub fn is_item_count_reached(&self) -> bool {
        if self.objectif.collecter.count == self.objectif.collecter.target {
            return true
        }
        false
    }

    // Retourne l'ID de l'ennemi à vaincre
    pub fn ennemi_id(&self) -> u32 {
        self.objectif.tuer.ennemi_id
    }

    // Incrémente le compteur d'ennemis vaincus
    pub fn inc_ennemi_count(&mut self) {
        self.objectif.tuer.count += 1;
    }

    // Vérifie si l'objectif de combat est atteint
    pub fn is_ennemi_count_reached(&self) -> bool {
        if self.objectif.tuer.count == self.objectif.tuer.target {
            return true
        }
        false
    }
}