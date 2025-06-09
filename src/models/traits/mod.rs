// Module de définition des traits principaux du jeu
// Ces traits définissent les comportements communs des différentes entités

// Trait permettant aux entités d'être décrites
// Utilisé pour afficher les informations sur les objets, personnages et lieux
pub mod descriptible;

// Trait définissant les capacités de combat
// Gère les attaques, la défense et les points de vie
pub mod combattant;

// Trait permettant aux entités d'interagir entre elles
// Gère les interactions entre le joueur et les autres entités
pub mod interactible;

// Trait de gestion de l'argent
// Permet aux entités de gérer leur argent (achat, vente, récompenses)
pub(crate) mod money_manager;

