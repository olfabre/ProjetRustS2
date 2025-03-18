use crate::models::{character::Character};

pub struct Combat{
    pub id: u32,
    pub Joueur: Character,
    pub joueurAdversaire: Character,
    pub tour: u8,
    pub actif: bool,

}

