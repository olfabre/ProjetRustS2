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
    pub jour: u32,          // Numéro du jour dans le jeu
    pub heure: u8,          // Heure actuelle (0-23)
    pub minute: u8,         // Minute actuelle (0-59)
    derniere_mise_a_jour: Instant,  // Moment de la dernière mise à jour du temps
}

// Énumération des différents états météorologiques possibles
// Affecte l'ambiance et potentiellement le gameplay
#[derive(Debug, Clone)]
pub enum Meteo {
    Clair,      // Temps ensoleillé et dégagé
    Pluvieux,   // Pluie modérée
    Orageux,    // Orages et pluies intenses
    Brumeux,    // Brouillard et visibilité réduite
}

// Énumération des différents types d'événements pouvant survenir dans le monde
// Permet de gérer les apparitions d'objets, déplacements de personnages et changements météo
pub enum Evenement {
    ObjetApparait(String, usize),      // Un objet apparaît dans une salle (nom de l'objet, ID de la salle)
    PersonnageDeplace(usize, usize),   // Un personnage se déplace (ID du personnage, ID de la salle de destination)
    ChangementMeteo(Meteo),            // La météo change (nouvel état météorologique)
}

// Structure principale représentant le monde du jeu
// Contient toutes les entités et états du monde (salles, personnages, objets, temps, météo)
pub struct worlds {
    pub salles: Vec<Room>,             // Liste des salles du monde
    pub personnages: Vec<Character>,   // Liste des personnages présents
    pub objets: Vec<Item>,             // Liste des objets disponibles
    pub temps: TempsJeu,               // État actuel du temps
    pub meteo_actuelle: Meteo,         // État météorologique actuel
    evenements: Vec<Evenement>,        // Liste des événements à traiter
}