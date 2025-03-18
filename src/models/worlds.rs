use std::time::{Instant, Duration};
use crate::models::room::Room;
use crate::models::character::Character;
use crate::models::item::Item;

pub struct TempsJeu{
    pub jour: u32,
    pub heure: u8,
    pub minute: u8, 
    derniere_mise_a_jour: Instant,
}

#[derive(Debug, Clone)]
pub enum Meteo {
    Clair,
    Pluvieux,
    Orageux,
    Brumeux,
}

pub enum Evenement {
    ObjetApparait(String, usize),  // nom de l'objet et ID de salle
    PersonnageDeplace(usize, usize),  // ID du personnage et ID de salle
    ChangementMeteo(Meteo),
}

pub struct worlds {
    pub salles: Vec<Room>,
    pub personnages: Vec<Character>,
    pub objets: Vec<Item>,
    pub temps: TempsJeu,
    pub meteo_actuelle: Meteo,
    evenements: Vec<Evenement>,
}