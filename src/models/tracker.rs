use std::collections::HashMap;
use crate::models::dialogue::{Dialogue, DialogueStep};
use crate::models::entities::character::Character;
use crate::models::entities::quete::Quete;

pub struct Tracker;


impl Tracker {

    /**
    Returns true or false on quest completion
    */
    pub fn item(item_id: u32,
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

    pub fn seaennemi(ennemi_id: u32,
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


    pub fn update_dialogues(id: u32, dialogues: &mut Vec<Dialogue>) {
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

/*
char -> quete
pnj -> dialogues
dialogues -> quete
quete -> dialogue


*/