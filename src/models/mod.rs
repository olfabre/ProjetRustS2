// Module principal contenant la logique métier du jeu
// Organise les différents composants du jeu en sous-modules

pub mod game;      // Gestion principale du jeu et de la boucle de jeu
pub mod dialogue;  // Système de dialogue et d'interaction avec les PNJ
mod combat;        // Système de combat entre le joueur et les ennemis
pub mod entities;  // Définition des entités du jeu (personnages, objets, etc.)
mod tracker;       // Système de suivi des quêtes et objectifs
pub mod traits;    // Traits définissant les comportements communs
//pub mod worlds;   // Gestion du monde du jeu (commenté pour le moment)
