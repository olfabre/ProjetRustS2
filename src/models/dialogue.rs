use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io::stdin;

use crate::models::entities::character::Character;
use crate::models::entities::quete::Quete;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue_id: u32,
    pub dialogue_steps: Vec<DialogueStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueStep {
    pub action: String,     // reference to quetes
    pub active: bool,
    pub question: String,
    pub options: Vec<DialogueOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueOption {
    pub réponse: String,
    pub réaction: String,
}

impl Dialogue {
    /// Affiche un dialogue et permet au joueur de choisir une réponse
    pub fn afficher_dialogue(&mut self, character: &mut Character, quetes: &mut HashMap<u32,Quete>) {
        if self.dialogue_steps.is_empty() {
            println!("Ce PNJ n'a rien à dire.");
        } else {

            let mut current_index = 0; // Start at the first dialogue step

            while current_index < self.dialogue_steps.len() {
                // let dialogue_step = &self.dialogues[current_index];
                let step = &self.dialogue_steps[current_index];
                // Instead of holding the mutable reference, extract needed values early
                let is_active = self.dialogue_steps[current_index].active;
                let question = self.dialogue_steps[current_index].question.clone();
                let options = self.dialogue_steps[current_index].options.clone();
                let action = self.dialogue_steps[current_index].action.clone();

                if !step.active {
                    current_index += 1;
                    continue;
                }

                println!("current index: ================> {}", current_index);
                println!("💬 PNJ : \"{}\"", question);

                for (i, option) in options.iter().enumerate() {
                    println!("{}. {}", i + 1, option.réponse);
                }

                println!("➡ Tape le numéro de ton choix :");
                let mut choix = String::new();
                stdin().read_line(&mut choix).expect("Erreur de lecture");

                if let Ok(index) = choix.trim().parse::<usize>() {
                    if index > 0 && index <= step.options.len() {
                        let selected_option = &options[index - 1];

                        // Afficher la réaction
                        println!("💬 PNJ : \"{}\"", selected_option.réaction);

                        // 🛠️ Vérifier si l'action commence par "accepteQuete"
                        if action.starts_with("accepteQuete") && selected_option.réponse == "Accepter Quête" {
                            // Extraire l'ID de la quête (si format "accepteQuete:42")
                            if let Some(id_str) = action.split(':').nth(1) {
                                if let Ok(id) = id_str.parse::<u32>() {
                                    println!("what happened ? ");
                                    character.ajouter_quete(id);
                                    println!("what happened ? 222");
                                    // Once the quest is accepted, the dialogue becomes inactive,
                                    self.dialogue_steps[current_index].active = false;
                                }
                            }
                        }


                        // come back here
                        //

                        // 🔍 Rechercher si la réaction correspond à une question existante
                        // if let Some((new_index, _)) = self.dialogue_steps
                        //     .iter()
                        //     .enumerate()
                        //     .find(|(_, d)| d.question == selected_option.réaction)
                        // {
                        //     current_index = new_index; // ✅ Mettre à jour l'index
                        //     continue; // 🔄 Passer à la prochaine question
                        // }

                        if selected_option.réponse.starts_with("Au revoir") ||
                            selected_option.réponse.starts_with("Ignorer") ||
                            selected_option.réponse.starts_with("Refuser") {
                            break;
                        }


                        // println!(" i guess i broke");
                        // 🛑 Fin du dialogue si aucune nouvelle question ne correspond
                        current_index += 1;
                    } else {
                        println!("❌ Choix invalide !");
                    }
                } else {
                    println!("❌ Entrée invalide !");
                }
            }
        }
    }
}
