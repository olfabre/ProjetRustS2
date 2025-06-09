// Trait définissant les capacités d'interaction pour les entités du jeu
// Permet aux entités d'être interagies par le joueur ou d'autres entités
pub trait Interactable {
    // Définit le comportement d'interaction de l'entité
    // Cette méthode est appelée lorsque le joueur interagit avec l'entité
    fn interact(&self);
}