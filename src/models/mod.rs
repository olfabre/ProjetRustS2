// Module principal contenant tous les modèles du jeu
// Ce module regroupe les différentes parties du système de jeu et définit leur organisation
// Il sert de point d'entrée pour accéder à toutes les fonctionnalités du jeu

// Module principal du jeu qui gère la logique et l'état du jeu
// Contient la structure Game qui coordonne toutes les autres composantes
pub mod game;

// Module de gestion des dialogues et des conversations
// Gère les interactions avec les PNJ et les choix de dialogue
pub mod dialogue;

// Module de gestion du système de combat
// Définit les règles de combat et les interactions entre combattants
mod combat;

// Module contenant toutes les entités du jeu (personnages, objets, etc.)
// Organise les différentes structures qui représentent les éléments du jeu
pub mod entities;

// Module de suivi des quêtes et des objectifs
// Permet de gérer la progression des quêtes et leurs objectifs
mod tracker;

// Module contenant les traits communs aux entités
// Définit les comportements partagés entre différentes entités
pub mod traits;

// Module de gestion des mondes et des zones (commenté pour le moment)
// Sera utilisé pour gérer les différents mondes et zones du jeu
//pub mod worlds;
