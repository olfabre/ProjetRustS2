// Module de suivi des quêtes et des objectifs
// Permet de gérer la progression des quêtes et la mise à jour des dialogues associés

use std::collections::HashMap;
use crate::models::dialogue::{Dialogue, DialogueStep};
use crate::models::entities::character::Character;
use crate::models::entities::quete::Quete;

// Trait définissant les fonctionnalités de suivi des quêtes
// Permet de suivre la progression des objectifs (objets collectés, ennemis vaincus)
pub trait Tracker {
    // Vérifie si un objet collecté correspond à un objectif de quête
    // Met à jour le compteur d'objets et active le dialogue de rendu si la quête est terminée
    // Retourne true si une quête a été complétée
    fn track_item(item_id: u32,
                  joueur: &mut Character,
                  quetes: &mut HashMap<u32, Quete>,
                  dialogues: &mut Vec<Dialogue>) -> bool {
        // Parcourt toutes les quêtes du joueur
        for quest_id in joueur.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) {
                // Vérifie si l'objet correspond à un objectif de quête
                if quest.item_id() == item_id {
                    // Incrémente le compteur d'objets
                    quest.inc_item_count();
                    
                    // Vérifie si l'objectif est atteint
                    if quest.is_item_count_reached() {
                        // Active le dialogue de rendu de quête
                        Self::update_dialogues(quest.dialog_rendu_id, dialogues);

                        println!("✅ Quête: {} est complete.", quest.name());
                        println!("Retournez voir le donneur de quête pour récupérer votre prix");
                        return true;
                    }
                }
            }
        }
        false
    }

    // Vérifie si un ennemi vaincu correspond à un objectif de quête
    // Met à jour le compteur d'ennemis et active le dialogue de rendu si la quête est terminée
    fn track_enemy(ennemi_id: u32,
                   joueur: &mut Character,
                   quetes: &mut HashMap<u32, Quete>,
                   dialogues: &mut Vec<Dialogue>) {
        // Parcourt toutes les quêtes du joueur
        for quest_id in joueur.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) {
                // Vérifie si l'ennemi correspond à un objectif de quête
                if quest.ennemi_id() == ennemi_id {
                    // Incrémente le compteur d'ennemis
                    quest.inc_ennemi_count();
                    
                    // Vérifie si l'objectif est atteint
                    if quest.is_ennemi_count_reached() {
                        // Active le dialogue de rendu de quête
                        Self::update_dialogues(quest.dialog_rendu_id, dialogues);

                        println!("✅ Quête: {} est complete.", quest.name());
                        println!("Retournez voir le donneur de quête pour récupérer votre prix");
                    }
                }
            }
        }
    }

    // Met à jour les dialogues associés à une quête terminée
    // Active les options de dialogue permettant de rendre la quête
    fn update_dialogues(id: u32, dialogues: &mut Vec<Dialogue>) {
        // Recherche le dialogue correspondant à l'ID
        let Some(mut dialogue) = dialogues.iter_mut().find(|mut dialog| {
            dialog.dialogue_id == id
        }) else { return };
        
        // Active les étapes de dialogue liées au rendu de quête
        for step in &mut dialogue.dialogue_steps {
            if step.action.starts_with("rendreQuete") {
                step.active = true;
            }
        }
    }
}

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