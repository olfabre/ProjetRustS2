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
                    continue; // 🔄 Passer à la prochaine question
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
                                    // let  quete = quetes.get(&id).unwrap();
                                    println!("🎯 Quête ajoutée : {}!", quetes.get(&id).unwrap().name());

                                    // Quand la quete est acceptée, le dialogue pour l'offrir disparait
                                    self.dialogue_steps[current_index].active = false;
                                }
                            }
                        }

                        // Verifier si action  == rendreQuete
                        // Verifier si reponse == Completer Quete
                         if action.starts_with("rendreQuete") && selected_option.réponse == "Completer Quête" {
                             // Split string a get the part after :
                             if let Some(id_str) = action.split(':').nth(1) {
                                 // parse string into u32
                                 if let Ok(id) = id_str.parse::<u32>() {
                                     // retrieve mut quete from Map
                                     let  quete = quetes.get(&id).unwrap();
                                     // Character supprimer quete
                                     character.supprimer_quete(id);
                                     character.add_experience(quete.experience);

                                     // On récupère l'objet depuis la liste globale
                                     for recompense_item in quete.recompense_items.iter() {
                                         if let Some(item) = items.iter().find(|item| item.id() == *recompense_item) {
                                             // On l'ajoute à l'inventaire du personnage
                                             character.inventory_mut().add_item(item.id(), 1);
                                             println!("👜 Tu as ramassé '{}'.", item.name());
                                         }
                                     }

                                     if quete.recompense_argent > 0 {
                                         println!("💰 Tu as gagné {} pièces d'argent.", quete.recompense_argent);
                                         character.add_argent(quete.recompense_argent);
                                     }
                                 }
                             }
                         }


                        // Sortir de la boucle tôt
                        if selected_option.réponse.starts_with("Au revoir") ||
                            selected_option.réponse.starts_with("Ignorer") ||
                            selected_option.réponse.starts_with("Refuser") {
                            break;
                        }

                        // Le but c'est montrer tous les steps qui sont actives
                        current_index += 1; // ✅ Mettre à jour l'index

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
