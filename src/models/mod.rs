// Module principal contenant tous les modèles du jeu
// Ce module regroupe les différentes parties du système de jeu

// Module principal du jeu qui gère la logique et l'état du jeu
pub mod game;

// Module de gestion des dialogues et des conversations
pub mod dialogue;

// Module de gestion du système de combat
mod combat;
// Module contenant toutes les entités du jeu (personnages, objets, etc.)
pub mod entities;
// Module de suivi des quêtes et des objectifs
mod tracker;
// Module contenant les traits communs aux entités
pub mod traits;
// Module de gestion des mondes et des zones (commenté pour le moment)
//pub mod worlds;
