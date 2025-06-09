// Trait définissant les capacités de description pour les entités du jeu
// Permet aux entités d'avoir une description textuelle visible par le joueur
pub trait Descriptible {
    // Retourne la description textuelle de l'entité
    // Cette description peut être affichée dans l'interface du jeu
    fn get_description(&self) -> String;
}