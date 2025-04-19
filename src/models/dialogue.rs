use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io::stdin;

use crate::models::entities::character::Character;
use crate::models::entities::item::Item;
use crate::models::entities::quete::Quete;
use crate::models::tracker::Tracker;

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
    // Affiche un dialogue et permet au joueur de choisir une réponse
    pub fn afficher_dialogue(&mut self, character: &mut Character,
                             quetes: &mut HashMap<u32,Quete>, items: &Vec<Item>) {
        if self.dialogue_steps.is_empty() {
            println!("Ce PNJ n'a rien à dire.");
        } else {

            let mut current_index = 0; // Start at the first dialogue step

            while current_index < self.dialogue_steps.len() {

                // This is a reference, because step is going to be changed
                let step = &self.dialogue_steps[current_index];

                // Instead of holding the mutable reference, extract needed values early

                let options = self.dialogue_steps[current_index].options.clone();
                let action = self.dialogue_steps[current_index].action.clone();

                // Si un step est inactive, on le saute
                if !step.active {
                    current_index += 1;
                    continue;
                }

                println!("current index: ================> {}", current_index);
                println!("💬 PNJ : \"{}\"", step.question);

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

                                    character.ajouter_quete(id);

                                    // Quand la quete est acceptée, le dialogue pour l'offrir disparait
                                    self.dialogue_steps[current_index].active = false;
                                }
                            }
                        }

                        // Verifier si rendreQuete action
                        // Verifier si reponse == Completer Quete
                         if action.starts_with("rendreQuete") && selected_option.réponse == "Completer Quête" {
                             if let Some(id_str) = action.split(':').nth(1) {
                                 if let Ok(id) = id_str.parse::<u32>() {
                                     let mut quete = quetes.get_mut(&id).unwrap();
                                     // Character supprimer quete
                                     character.supprimer_quete(id);
                                     character.add_experience(quete.experience);

                                     // On récupère l'objet depuis la liste globale
                                     for recompense_item in quete.recompense_items.iter() {
                                         if let Some(item) = items.iter().find(|item| item.id() == *recompense_item) {
                                             // On l'ajoute à l'inventaire du personnage
                                             character.inventory_mut().push(item.clone());
                                             println!("👜 Tu as ramassé '{}'.", item.name());
                                         }
                                     }
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

                        // Sortir de la boucle tôt
                        if selected_option.réponse.starts_with("Au revoir") ||
                            selected_option.réponse.starts_with("Ignorer") ||
                            selected_option.réponse.starts_with("Refuser") {
                            break;
                        }

                        // Le but c'est montrer tous les steps qui sont actives
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
