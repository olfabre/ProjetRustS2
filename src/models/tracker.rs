use std::collections::HashMap;
use crate::models::dialogue::{Dialogue, DialogueStep};
use crate::models::entities::character::Character;
use crate::models::entities::quete::Quete;

pub trait Tracker {

    /// Lors de l'ajout d'un objet à l'inventaire, cette fonction est appelée et vérifie si l'objet
    /// est un objet nécessaire pour terminer une quête
    /// Renvoie true ou false
    fn track_item(item_id: u32,
                  joueur: &mut Character,
                  quetes: &mut HashMap<u32, Quete>,
                  dialogues: &mut Vec<Dialogue>) -> bool {
        for quest_id in joueur.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) {
                if quest.item_id() == item_id {

                    // Augmenter le nombre d'objets pour la quête
                    quest.inc_item_count();
                    if quest.is_item_count_reached() { // Vérifiez si le nombre d'éléments requis a été atteint

                        // Mettre à jour le dialogue correspondant pour refléter l'état d'achèvement de la quête
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

    /// Lors de tuer un ennemi, cette fonction est appelée et vérifie si l'ennemi
    /// est nécessaire pour terminer une quête
    /// Renvoie true ou false
    fn track_enemy(ennemi_id: u32,
                   joueur: &mut Character,
                   quetes: &mut HashMap<u32, Quete>,
                   dialogues: &mut Vec<Dialogue>) {

        for quest_id in joueur.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) { // Accéder à la quête en utilisant son ID
                if quest.ennemi_id() == ennemi_id {

                    // Augmente le nombre d'ennemis requis pour la quête
                    quest.inc_ennemi_count();
                    if quest.is_ennemi_count_reached() { // Check if enough enemies have been defeated

                        // Mettre à jour le dialogue pour l'état d'achèvement de la quête
                        Self::update_dialogues(quest.dialog_rendu_id, dialogues);

                        // Notifier le joueur que la quête est terminée
                        println!("✅ Quête: {} est complete.", quest.name());
                        println!("Retournez voir le donneur de quête pour récupérer votre prix");

                    }
                }
            }
        }
    }

    /// Si une quête est terminée, les étapes de dialogue doivent changer pour que le joueur puisse
    /// rendre la quête
    fn update_dialogues(id: u32, dialogues: &mut Vec<Dialogue>) {
        // Rechercher le dialogue associé à l'ID donné
        let Some(mut dialogue) = dialogues.iter_mut().find(|mut dialog| {
            dialog.dialogue_id == id
        }) else { return };

        // Parcourez les étapes du dialogue et activez toute étape correspondant à l'achèvement d'une quête
        for step in &mut dialogue.dialogue_steps {
            if step.action.starts_with("rendreQuete") { // Vérifiez si l'action fait référence à l'achèvement d'une quête
                step.active = true; // Activez l'étape pour qu'elle fasse partie des options de dialogue du joueur
            }
        }
    }
}

impl Tracker for Character {

}