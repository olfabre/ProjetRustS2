use std::collections::HashMap;
use crate::models::dialogue::{Dialogue, DialogueStep};
use crate::models::entities::character::Character;
use crate::models::entities::quete::Quete;

pub struct Tracker;


impl Tracker {

    /**
    Returns true or false on quest completion
    */
    pub fn item(item_id: u32, char: &mut Character, quetes: &mut HashMap<u32, Quete>, dialogues: &mut Vec<Dialogue>) -> bool {
        for quest_id in char.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) { // Use `get_mut` for mutable access
                if quest.item_id() == item_id {

                    // increment the count, AND verify right after
                    quest.inc_item_count();
                    if quest.verify_count() {
                        // quest.statu = String::from("complete");

                        // Make dialogue to turn in completed quests available
                        let Some(mut dialogue) = dialogues.iter_mut().find(|mut dialog| {
                            dialog.dialogue_id == quest.dialog_rendu_id
                        }) else { return false };
                        for step in &mut dialogue.dialogue_steps {
                            if step.action.starts_with("rendreQuete") {
                                step.active = true;
                            }
                        }

                        println!("✅ Quête: {} est complete.", quest.name());
                        println!("Retournez voir le donneur de quête pour récupérer votre prix");
                        return true;
                    }
                }
            }
        }
        false
    }



}

/*
char -> quete
pnj -> dialogues
dialogues -> quete
quete -> dialogue


*/