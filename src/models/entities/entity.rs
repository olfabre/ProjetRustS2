// Module définissant la structure de base pour toutes les entités du jeu
// Fournit les attributs et méthodes communs à toutes les entités

use serde::{Deserialize, Serialize};

// Structure de base pour toutes les entités du jeu
// Contient les attributs fondamentaux : identifiant, nom et description
// Implémente Serialize et Deserialize pour la persistance des données
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    id: u32,              // Identifiant unique de l'entité
    name: String,         // Nom de l'entité
    description: String,  // Description détaillée de l'entité
}

impl Entity {
    // Getters pour accéder aux attributs de l'entité
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    // Setters pour modifier les attributs de l'entité
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}