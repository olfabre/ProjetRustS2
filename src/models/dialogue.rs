// === Imports ===
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io::stdin;

// Internal module imports (domain models and traits)
use crate::models::entities::character::Character;
use crate::models::entities::item::Item;
use crate::models::entities::pnj::Pnj;
use crate::models::entities::quete::Quete;
use crate::models::traits::money_manager::MoneyManager;

// === Dialogue Structure ===
// Represents an interactive conversation with a PNJ (NPC)

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue_id: u32,                  // Unique ID for the dialogue
    pub dialogue_steps: Vec<DialogueStep>, // Sequence of steps/questions in the dialogue
}

// === Dialogue Step ===
// A single step in a dialogue (a question and options)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueStep {
    pub action: String,     // Associated action (e.g., accept quest)
    pub active: bool,       // Whether this step is currently active
    pub question: String,   // The question asked to the player
    pub options: Vec<DialogueOption>, // List of answer choices
}

// === Dialogue Option ===
// A possible response to a dialogue step, with NPC reaction
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueOption {
    pub réponse: String,    // Player's response
    pub réaction: String,   // NPC's reaction to the response
}

// === Dialogue Implementation ===
impl Dialogue {

    // === afficher_dialogue ===
    // Handles the interactive dialogue with the player
    // - Displays dialogue questions
    // - Handles quest acceptance/completion
    // - Triggers merchant mode if needed
    pub fn afficher_dialogue(&mut self, character: &mut Character,
                             quetes: &mut HashMap<u32,Quete>, items: &Vec<Item>, pnj: &mut Pnj) {
        if self.dialogue_steps.is_empty() {
            println!("Ce PNJ n'a rien à dire.");
        } else {

            let mut current_index = 0; // Index of the current dialogue step

            while current_index < self.dialogue_steps.len() {

                let step = &self.dialogue_steps[current_index];
                let options = self.dialogue_steps[current_index].options.clone();
                let action = self.dialogue_steps[current_index].action.clone();

                // Skip inactive steps
                if !step.active {
                    current_index += 1;
                    continue;
                }

                // === Display the dialogue step and options ===
                println!("current index: ================> {}", current_index);
                println!("💬 PNJ : \"{}\"", step.question);
                for (i, option) in options.iter().enumerate() {
                    println!("{}. {}", i + 1, option.réponse);
                }

                // === Read user input ===
                println!("➡ Tape le numéro de ton choix :");
                let mut choix = String::new();
                stdin().read_line(&mut choix).expect("Erreur de lecture");

                // === Handle user response ===
                if let Ok(index) = choix.trim().parse::<usize>() {
                    if index > 0 && index <= step.options.len() {
                        let selected_option = &options[index - 1];
                        println!("💬 PNJ : \"{}\"", selected_option.réaction);

                        // === Accept quest ===
                        if action.starts_with("accepteQuete") && selected_option.réponse == "Accepter Quête" {
                            if let Some(id_str) = action.split(':').nth(1) {
                                if let Ok(id) = id_str.parse::<u32>() {
                                    character.ajouter_quete(id);
                                    println!("🎯 Quête ajoutée : {}!", quetes.get(&id).unwrap().name());
                                    self.dialogue_steps[current_index].active = false;
                                }
                            }
                        }

                        // === Complete quest ===
                        if action.starts_with("rendreQuete") && selected_option.réponse == "Completer Quête" {
                            if let Some(id_str) = action.split(':').nth(1) {
                                if let Ok(id) = id_str.parse::<u32>() {
                                    let quete = quetes.get(&id).unwrap();

                                    if quete.objectif_type == "collecter" {
                                        character.inventory_mut().remove_item(quete.objectif.collecter.item_id, quete.objectif.collecter.target);
                                        pnj.inventory_mut().add_item(quete.objectif.collecter.item_id, quete.objectif.collecter.target);
                                    }

                                    character.supprimer_quete(id);
                                    character.add_experience(quete.experience);

                                    // === Distribute reward items ===
                                    for recompense_item in quete.recompense_items.iter() {
                                        if let Some(item) = items.iter().find(|item| item.id() == *recompense_item) {
                                            character.inventory_mut().add_item(item.id(), 1);
                                            println!("👜 Tu as ramassé '{}'.", item.name());
                                        }
                                    }

                                    // === Distribute money reward ===
                                    if quete.recompense_argent > 0 {
                                        println!("🪙 Tu as gagné {} pièces d'money.", quete.recompense_argent);
                                        character.add_money(quete.recompense_argent);
                                    }
                                }
                            }
                        }

                        // === Start merchant system ===
                        if action.starts_with("merchant") && selected_option.réponse == "Commerce" {
                            self.start_merchant(character, items, pnj);
                        }

                        // === Exit dialogue early if player says goodbye ===
                        if selected_option.réponse.starts_with("Au revoir") ||
                            selected_option.réponse.starts_with("Ignorer") ||
                            selected_option.réponse.starts_with("Refuser") {
                            break;
                        }

                        // Move to next step
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

    // === start_merchant ===
    // Handles buying/selling items between character and PNJ
    pub fn start_merchant(&mut self, character: &mut Character, items: &Vec<Item>, pnj: &mut Pnj) {
        loop {
            println!("\n👤 {} (🪙 {})", character.name(), character.money);
            println!("🛒 Marchand (🪙 {}) : \"Voici mes merchandises.\"", pnj.money);
            let mut merchant_items = pnj.inventory_mut();

            // === List merchant items ===
            for (i, inventory_item) in merchant_items.items.iter().enumerate() {
                let id = inventory_item.item_id;
                if let Some(item) = items.iter().find(|i| i.id() == id) {
                    println!("{}. {} - 🪙 {} - Qt: {}", i + 1, item.name(), item.value, inventory_item.quantity);
                } else {
                    println!("{}. Objet inconnu (ID: {})", i + 1, id);
                }
            }

            println!("vendre <objet>, inventaire (afficher inventaire)");
            println!("quitter");
            println!("\n➡ Tapez le numéro de l'objet à acheter, ou autre choix :");

            let mut choix = String::new();
            stdin().read_line(&mut choix).expect("Erreur de lecture");

            // === Quit merchant mode ===
            if choix.trim().eq_ignore_ascii_case("quitter") {
                println!("👋 Bon Affaire");
                break;
            }

            // === Buying items ===
            if let Ok(index) = choix.trim().parse::<usize>() {
                if index > 0 && index <= merchant_items.items.len() {
                    let inventory_item = &merchant_items.items[index - 1];
                    let Some(item) = items.iter().find(|i| i.id() == inventory_item.item_id)
                    else { todo!() };

                    if character.money >= item.value {
                        character.inventory_mut().add_item(item.id(), 1);
                        character.remove_money(item.value);
                        pnj.inventory_mut().remove_item(item.id(), 1);
                        pnj.add_money(item.value);
                        println!("🪙 Tu as acheté '{}'.", item.name());
                    } else {
                        println!("❌ Pas assez d'argent !");
                    }
                } else {
                    println!("❌ Choix invalide !");
                }
            }

            // === Selling items ===
            else if choix.starts_with("vendre ") {
                let objet_nom = &choix[7..].trim();
                if let Some(item) = items.iter().find(|i| {
                    i.name().eq_ignore_ascii_case(objet_nom) &&
                        character.inventory_mut().items.iter().any(|(inv)| inv.item_id == i.id())
                }) {
                    if pnj.money >= item.value {
                        character.inventory_mut().remove_item(item.id(), 1);
                        character.add_money(item.value);
                        pnj.inventory_mut().add_item(item.id(), 1);
                        pnj.remove_money(item.value);
                        println!("🪙 Tu as vendu '{}'.", item.name());
                    } else {
                        println!("❌ Tu n'as pas cet objet dans ton inventaire !");
                    }
                } else {
                    println!("❌ Objet non trouvé dans ton inventaire !");
                }

            // === Show inventory ===
            } else if choix.starts_with("inventaire") {
                character.afficher_inventaire(items);
            } else {
                println!("❌ Entrée invalide !");
            }
        }
    }

}
