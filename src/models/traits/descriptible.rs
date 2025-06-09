// Trait permettant aux entités d'être décrites
// Utilisé pour afficher des informations détaillées sur les objets, personnages et lieux
pub trait Descriptible {
    // Retourne une description textuelle de l'entité
    // Cette description peut inclure des informations sur l'apparence, l'état, les caractéristiques, etc.
    fn get_description(&self) -> String;
}