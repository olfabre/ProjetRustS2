use crate::models::elements::character::Character;

pub struct Combat{
    pub id: u32,
    pub joueur: Character,
    pub joueur_adversaire: Character,
    pub tour: u8,
    pub actif: bool,

}
