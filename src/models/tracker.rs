// Module de suivi des quêtes et des objectifs
// Permet de gérer la progression des quêtes et la mise à jour des dialogues associés

use std::collections::HashMap;
use crate::models::dialogue::{Dialogue, DialogueStep};
use crate::models::entities::character::Character;
use crate::models::entities::quete::Quete;

// Trait définissant les fonctionnalités de suivi des quêtes
// Permet de suivre la progression des objectifs (objets collectés, ennemis vaincus)
pub trait Tracker {
<<<<<<< HEAD
    // Vérifie si un objet collecté correspond à un objectif de quête
    // Met à jour le compteur d'objets et active le dialogue de rendu si la quête est terminée
    // Retourne true si une quête a été complétée
=======

    /// Lors de l'ajout d'un objet à l'inventaire, cette fonction est appelée et vérifie si l'objet
    /// est un objet nécessaire pour terminer une quête
    /// Renvoie true ou false
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
    fn track_item(item_id: u32,
                  joueur: &mut Character,
                  quetes: &mut HashMap<u32, Quete>,
                  dialogues: &mut Vec<Dialogue>) -> bool {
        // Parcourt toutes les quêtes du joueur
        for quest_id in joueur.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) {
<<<<<<< HEAD
                // Vérifie si l'objet correspond à un objectif de quête
                if quest.item_id() == item_id {
                    // Incrémente le compteur d'objets
                    quest.inc_item_count();
                    
                    // Vérifie si l'objectif est atteint
                    if quest.is_item_count_reached() {
                        // Active le dialogue de rendu de quête
=======
                if quest.item_id() == item_id {

                    // Augmenter le nombre d'objets pour la quête
                    quest.inc_item_count();
                    if quest.is_item_count_reached() { // Vérifiez si le nombre d'éléments requis a été atteint

                        // Mettre à jour le dialogue correspondant pour refléter l'état d'achèvement de la quête
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                        Self::update_dialogues(quest.dialog_rendu_id, dialogues);

                        // Notifier le joueur que la quête est terminée
                        println!("✅ Quête: {} est complete.", quest.name());
                        println!("Retournez voir le donneur de quête pour récupérer votre prix");
                        return true;
                    }
                }
            }
        }
        false // Si quête n'est pas complete
    }

<<<<<<< HEAD
    // Vérifie si un ennemi vaincu correspond à un objectif de quête
    // Met à jour le compteur d'ennemis et active le dialogue de rendu si la quête est terminée
=======
    /// Lors de tuer un ennemi, cette fonction est appelée et vérifie si l'ennemi
    /// est nécessaire pour terminer une quête
    /// Renvoie true ou false
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
    fn track_enemy(ennemi_id: u32,
                   joueur: &mut Character,
                   quetes: &mut HashMap<u32, Quete>,
                   dialogues: &mut Vec<Dialogue>) {
        // Parcourt toutes les quêtes du joueur
        for quest_id in joueur.quests() {
<<<<<<< HEAD
            if let Some(quest) = quetes.get_mut(&quest_id) {
                // Vérifie si l'ennemi correspond à un objectif de quête
                if quest.ennemi_id() == ennemi_id {
                    // Incrémente le compteur d'ennemis
                    quest.inc_ennemi_count();
                    
                    // Vérifie si l'objectif est atteint
                    if quest.is_ennemi_count_reached() {
                        // Active le dialogue de rendu de quête
=======
            if let Some(quest) = quetes.get_mut(&quest_id) { // Accéder à la quête en utilisant son ID
                if quest.ennemi_id() == ennemi_id {

                    // Augmente le nombre d'ennemis requis pour la quête
                    quest.inc_ennemi_count();
                    if quest.is_ennemi_count_reached() { // Check if enough enemies have been defeated

                        // Mettre à jour le dialogue pour l'état d'achèvement de la quête
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                        Self::update_dialogues(quest.dialog_rendu_id, dialogues);

                        // Notifier le joueur que la quête est terminée
                        println!("✅ Quête: {} est complete.", quest.name());
                        println!("Retournez voir le donneur de quête pour récupérer votre prix");
                    }
                }
            }
        }
    }

<<<<<<< HEAD
    // Met à jour les dialogues associés à une quête terminée
    // Active les options de dialogue permettant de rendre la quête
    fn update_dialogues(id: u32, dialogues: &mut Vec<Dialogue>) {
        // Recherche le dialogue correspondant à l'ID
        let Some(mut dialogue) = dialogues.iter_mut().find(|mut dialog| {
            dialog.dialogue_id == id
        }) else { return };
        
        // Active les étapes de dialogue liées au rendu de quête
=======
    /// Si une quête est terminée, les étapes de dialogue doivent changer pour que le joueur puisse
    /// rendre la quête
    fn update_dialogues(id: u32, dialogues: &mut Vec<Dialogue>) {
        // Rechercher le dialogue associé à l'ID donné
        let Some(mut dialogue) = dialogues.iter_mut().find(|mut dialog| {
            dialog.dialogue_id == id
        }) else { return };

        // Parcourez les étapes du dialogue et activez toute étape correspondant à l'achèvement d'une quête
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
        for step in &mut dialogue.dialogue_steps {
            if step.action.starts_with("rendreQuete") { // Vérifiez si l'action fait référence à l'achèvement d'une quête
                step.active = true; // Activez l'étape pour qu'elle fasse partie des options de dialogue du joueur
            }
        }
    }
}

<<<<<<< HEAD
// Relations entre les différentes entités du système de quêtes :
// - Character -> Quête : Un personnage peut avoir plusieurs quêtes
// - PNJ -> Dialogues : Un PNJ peut avoir plusieurs dialogues
// - Dialogues -> Quête : Les dialogues peuvent être liés à des quêtes
// - Quête -> Dialogue : Une quête peut avoir des dialogues spécifiques

/*
char -> quete
pnj -> dialogues
dialogues -> quete
quete -> dialogue


*/
=======
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
