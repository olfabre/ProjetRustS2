// Module de suivi des quêtes et des objectifs
// Permet de gérer la progression des quêtes et la mise à jour des dialogues associés
// Ce module est responsable du suivi de la progression des quêtes et de l'activation des dialogues

use std::collections::HashMap;
use crate::models::dialogue::{Dialogue, DialogueStep};
use crate::models::entities::character::Character;
use crate::models::entities::quete::Quete;

// Trait définissant les fonctionnalités de suivi des quêtes
// Permet de suivre la progression des objectifs (objets collectés, ennemis vaincus)
// Ce trait est implémenté par les entités qui doivent suivre la progression des quêtes
pub trait Tracker {
    // Vérifie si un objet collecté correspond à un objectif de quête
    // Met à jour le compteur d'objets et active le dialogue de rendu si la quête est terminée
    // Retourne true si une quête a été complétée
    fn track_item(item_id: u32,
                  joueur: &mut Character,
                  quetes: &mut HashMap<u32, Quete>,
                  dialogues: &mut Vec<Dialogue>) -> bool {
        for quest_id in joueur.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) { // Use `get_mut` for mutable access
                if quest.item_id() == item_id {

                    // increment the count, AND verify right after
                    quest.inc_item_count();
                    if quest.is_item_count_reached() {

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
    // Cette méthode est appelée après chaque combat réussi
    fn track_enemy(ennemi_id: u32,
                   joueur: &mut Character,
                   quetes: &mut HashMap<u32, Quete>,
                   dialogues: &mut Vec<Dialogue>) {

        for quest_id in joueur.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) {
                if quest.ennemi_id() == ennemi_id {
                    quest.inc_ennemi_count();
                    if quest.is_ennemi_count_reached() {


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
    // Cette méthode est appelée lorsqu'une quête est complétée
    fn update_dialogues(id: u32, dialogues: &mut Vec<Dialogue>) {
        let Some(mut dialogue) = dialogues.iter_mut().find(|mut dialog| {
            dialog.dialogue_id == id
        }) else { return  };
        for step in &mut dialogue.dialogue_steps {
            if step.action.starts_with("rendreQuete") {
                step.active = true;
            }
        }

    }

}

// Relations entre les différentes entités du système de quêtes :
// - Character -> Quête : Un personnage peut avoir plusieurs quêtes actives
// - PNJ -> Dialogues : Un PNJ peut avoir plusieurs dialogues disponibles
// - Dialogues -> Quête : Les dialogues peuvent être liés à des quêtes spécifiques
// - Quête -> Dialogue : Une quête peut avoir des dialogues de début et de fin

/*
char -> quete
pnj -> dialogues
dialogues -> quete
quete -> dialogue


*/