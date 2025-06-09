// Trait permettant aux entités d'interagir avec le joueur
// Définit le comportement d'interaction pour les objets, PNJ et autres éléments interactifs
pub trait Interactable {
    // Définit l'action à effectuer lorsque le joueur interagit avec l'entité
    // Cette méthode peut déclencher des dialogues, des événements ou modifier l'état du jeu
    fn interact(&self);
}