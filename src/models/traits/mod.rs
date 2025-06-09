// Module regroupant tous les traits du jeu
// Ces traits définissent les comportements communs que peuvent avoir les entités du jeu
// Chaque trait peut être implémenté par différentes entités selon leurs besoins

// Trait pour les entités qui peuvent être décrites
pub mod descriptible;
// Trait pour les entités qui peuvent combattre
pub mod combattant;
// Trait pour les entités avec lesquelles on peut interagir
pub mod interactible;
// Trait pour les entités qui peuvent gérer de l'argent
pub(crate) mod money_manager;

