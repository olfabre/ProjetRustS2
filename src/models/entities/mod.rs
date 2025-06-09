// Module définissant toutes les entités du jeu
// Organise les différents types d'entités et leurs composants

pub mod entity;        // Classe de base pour toutes les entités du jeu
pub mod item;          // Gestion des objets et items du jeu
pub mod vivant;        // Entités vivantes (base pour personnages et ennemis)
pub mod character;     // Personnage joueur et ses caractéristiques
pub mod pnj;          // Personnages non-joueurs
pub mod Enemy;         // Ennemis et créatures hostiles
pub mod room;          // Salles et zones du jeu
pub mod quete;         // Système de quêtes et objectifs
pub mod loot_entry;    // Gestion du butin et des récompenses
mod inventory;         // Système d'inventaire
mod inventory_item;    // Items dans l'inventaire

