// Module de base pour toutes les entités du jeu
// Définit la structure Entity qui sert de base pour tous les éléments du jeu
// Permet la sérialisation/désérialisation pour la sauvegarde

use serde::{Deserialize, Serialize};

// Structure de base pour toutes les entités du jeu
// Contient les propriétés fondamentales partagées par tous les éléments
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    id: u32,              // Identifiant unique de l'entité
    name: String,         // Nom de l'entité
    description: String,  // Description détaillée de l'entité
}

impl Entity {
    // Retourne l'identifiant unique de l'entité
    pub fn id(&self) -> u32 {
        self.id
    }

    // Retourne le nom de l'entité
    pub fn name(&self) -> &str {
        &self.name
    }

    // Retourne la description de l'entité
    pub fn description(&self) -> &str {
        &self.description
    }

    // Modifie l'identifiant de l'entité
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    // Modifie le nom de l'entité
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    // Modifie la description de l'entité
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}