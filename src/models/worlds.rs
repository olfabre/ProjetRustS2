// Module de gestion du monde du jeu
// Définit les structures et énumérations pour gérer le temps, la météo et les événements

use std::time::{Instant, Duration};
use crate::models::room::Room;
use crate::models::character::Character;
use crate::models::item::Item;

// Structure gérant le temps dans le jeu
// Permet de suivre les jours, heures et minutes du jeu
// Utilise Instant pour calculer les mises à jour du temps
pub struct TempsJeu {
    pub jour: u32,
    pub heure: u8,
    pub minute: u8, 
    derniere_mise_a_jour: Instant,
}

// Énumération des différents états météorologiques possibles
// Affecte l'ambiance et potentiellement le gameplay
#[derive(Debug, Clone)]
pub enum Meteo {
    Clair,
    Pluvieux,
    Orageux,
    Brumeux,
}

// Énumération des différents types d'événements pouvant survenir dans le monde
// Permet de gérer les apparitions d'objets, déplacements de personnages et changements météo
pub enum Evenement {
    ObjetApparait(String, usize),  // nom de l'objet et ID de salle
    PersonnageDeplace(usize, usize),  // ID du personnage et ID de salle
    ChangementMeteo(Meteo),
}

// Structure principale représentant le monde du jeu
// Contient toutes les entités et états du monde (salles, personnages, objets, temps, météo)
pub struct worlds {
    pub salles: Vec<Room>,
    pub personnages: Vec<Character>,
    pub objets: Vec<Item>,
    pub temps: TempsJeu,
    pub meteo_actuelle: Meteo,
    evenements: Vec<Evenement>,
}