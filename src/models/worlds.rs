// Module de gestion du monde du jeu
// Définit les structures et énumérations pour gérer le temps, la météo et les événements
// Ce module est responsable de la simulation du monde et de ses changements dynamiques

use std::time::{Instant, Duration};
use crate::models::room::Room;
use crate::models::character::Character;
use crate::models::item::Item;

// Structure gérant le temps dans le jeu
// Permet de suivre les jours, heures et minutes du jeu
// Utilise Instant pour calculer les mises à jour du temps
// Le temps peut affecter les événements et l'ambiance du jeu
pub struct TempsJeu {
    pub jour: u32,        // Numéro du jour dans le jeu
    pub heure: u8,        // Heure actuelle (0-23)
    pub minute: u8,       // Minute actuelle (0-59)
    derniere_mise_a_jour: Instant,  // Dernière mise à jour du temps
}

// Énumération des différents états météorologiques possibles
// Affecte l'ambiance et potentiellement le gameplay
// La météo peut influencer les combats, les déplacements et les événements
#[derive(Debug, Clone)]
pub enum Meteo {
    Clair,    // Temps normal, pas d'effets spéciaux
    Pluvieux, // Réduit la visibilité et peut affecter certains sorts
    Orageux,  // Conditions dangereuses, peut déclencher des événements spéciaux
    Brumeux,  // Réduit la visibilité et peut cacher des ennemis
}

// Énumération des différents types d'événements pouvant survenir dans le monde
// Permet de gérer les apparitions d'objets, déplacements de personnages et changements météo
// Les événements peuvent être programmés ou aléatoires
pub enum Evenement {
    ObjetApparait(String, usize),     // Un objet apparaît dans une salle spécifique
    PersonnageDeplace(usize, usize),  // Un personnage se déplace vers une autre salle
    ChangementMeteo(Meteo),           // La météo change
}

// Structure principale représentant le monde du jeu
// Contient toutes les entités et états du monde (salles, personnages, objets, temps, météo)
// Cette structure gère l'état global du monde et ses changements
pub struct worlds {
    pub salles: Vec<Room>,           // Toutes les salles du monde
    pub personnages: Vec<Character>, // Tous les personnages du monde
    pub objets: Vec<Item>,          // Tous les objets du monde
    pub temps: TempsJeu,            // État actuel du temps
    pub meteo_actuelle: Meteo,      // Météo actuelle
    evenements: Vec<Evenement>,     // Liste des événements à traiter
}