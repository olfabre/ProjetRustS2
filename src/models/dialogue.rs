// Module de gestion des dialogues et interactions avec les PNJ
// Permet de gérer les conversations, les quêtes et le commerce avec les personnages non-joueurs

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io::stdin;

use crate::models::entities::character::Character;
use crate::models::entities::item::Item;
use crate::models::entities::pnj::Pnj;
use crate::models::entities::quete::Quete;
use crate::models::tracker::Tracker;
use crate::models::traits::money_manager::MoneyManager;

// Structure représentant un dialogue complet avec un PNJ
// Contient un identifiant unique et une liste d'étapes de dialogue
#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue_id: u32,           // Identifiant unique du dialogue
    pub dialogue_steps: Vec<DialogueStep>,  // Liste des étapes du dialogue
}

// Structure représentant une étape de dialogue
// Contient une action associée, un état actif/inactif, une question et des options de réponse
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueStep {
    pub action: String,     // Action à effectuer (ex: accepteQuete, rendreQuete, merchant)
    pub active: bool,       // Indique si cette étape est disponible
    pub question: String,   // Question posée par le PNJ
    pub options: Vec<DialogueOption>,  // Liste des réponses possibles
}

// Structure représentant une option de réponse dans un dialogue
// Contient la réponse du joueur et la réaction du PNJ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueOption {
    pub réponse: String,    // Texte de la réponse du joueur
    pub réaction: String,   // Réaction du PNJ à cette réponse
}

impl Dialogue {
    // Affiche et gère un dialogue complet avec un PNJ
    // Permet au joueur de choisir des réponses et déclenche les actions associées
    // Gère les quêtes, le commerce et les interactions spéciales
    pub fn afficher_dialogue(&mut self, character: &mut Character,
                             quetes: &mut HashMap<u32,Quete>, items: &Vec<Item>, pnj: &mut Pnj) {
        // Vérifie si le PNJ a des dialogues disponibles
        if self.dialogue_steps.is_empty() {
            println!("Ce PNJ n'a rien à dire.");
        } else {
            let mut current_index = 0; // Commence par la première étape du dialogue

            // Parcourt toutes les étapes du dialogue
            while current_index < self.dialogue_steps.len() {
                let step = &self.dialogue_steps[current_index];
                let options = self.dialogue_steps[current_index].options.clone();
                let action = self.dialogue_steps[current_index].action.clone();

                // Ignore les étapes inactives
                if !step.active {
                    current_index += 1;
                    continue;
                }

                // Affiche la question et les options de réponse
                println!("current index: ================> {}", current_index);
                println!("💬 PNJ : \"{}\"", step.question);

                for (i, option) in options.iter().enumerate() {
                    println!("{}. {}", i + 1, option.réponse);
                }

                // Attend le choix du joueur
                println!("➡ Tape le numéro de ton choix :");
                let mut choix = String::new();
                stdin().read_line(&mut choix).expect("Erreur de lecture");

                // Traite le choix du joueur
                if let Ok(index) = choix.trim().parse::<usize>() {
                    if index > 0 && index <= step.options.len() {
                        let selected_option = &options[index - 1];
                        println!("💬 PNJ : \"{}\"", selected_option.réaction);

                        // Gère l'acceptation d'une quête
                        if action.starts_with("accepteQuete") && selected_option.réponse == "Accepter Quête" {
                            if let Some(id_str) = action.split(':').nth(1) {
                                if let Ok(id) = id_str.parse::<u32>() {
                                    character.ajouter_quete(id);
                                    println!("🎯 Quête ajoutée : {}!", quetes.get(&id).unwrap().name());
                                    self.dialogue_steps[current_index].active = false;
                                }
                            }
                        }

                        // Gère la complétion d'une quête
                        if action.starts_with("rendreQuete") && selected_option.réponse == "Completer Quête" {
                            if let Some(id_str) = action.split(':').nth(1) {
                                if let Ok(id) = id_str.parse::<u32>() {
                                    let quete = quetes.get(&id).unwrap();
                                    character.supprimer_quete(id);
                                    character.add_experience(quete.experience);

                                    // Ajoute les récompenses d'objets
                                    for recompense_item in quete.recompense_items.iter() {
                                        if let Some(item) = items.iter().find(|item| item.id() == *recompense_item) {
                                            character.inventory_mut().add_item(item.id(), 1);
                                            println!("👜 Tu as ramassé '{}'.", item.name());
                                        }
                                    }

                                    // Ajoute la récompense d'argent
                                    if quete.recompense_argent > 0 {
                                        println!("🪙 Tu as gagné {} pièces d'money.", quete.recompense_argent);
                                        character.add_money(quete.recompense_argent);
                                    }
                                }
                            }
                        }

                        // Démarre l'interface de commerce
                        if action.starts_with("merchant") && selected_option.réponse == "Commerce" {
                            self.start_merchant(character, items, pnj);
                        }

                        // Termine le dialogue si le joueur choisit de partir
                        if selected_option.réponse.starts_with("Au revoir") ||
                            selected_option.réponse.starts_with("Ignorer") ||
                            selected_option.réponse.starts_with("Refuser") {
                            break;
                        }

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

    // Démarre l'interface de commerce avec un marchand
    // Permet au joueur d'acheter et vendre des objets
    // Gère l'argent et l'inventaire du joueur et du marchand
    pub fn start_merchant(&mut self, character: &mut Character, items: &Vec<Item>, pnj: &mut Pnj) {
        loop {
            // Affiche les informations du joueur et du marchand
            println!("\n👤 {} (🪙 {})", character.name(), character.money);
            println!("🛒 Marchand (🪙 {}) : \"Voici mes merchandises.\"", pnj.money);
            let mut merchant_items = pnj.inventory_mut();

            // Affiche la liste des objets disponibles
            for (i, inventory_item) in merchant_items.items.iter().enumerate() {
                let id = inventory_item.item_id;
                if let Some(item) = items.iter().find(|i| i.id() == id) {
                    println!("{}. {} - 🪙 {} - Qt: {}", i + 1, item.name(), item.value, inventory_item.quantity) ;
                } else {
                    println!("{}. Objet inconnu (ID: {})", i + 1, id);
                }
            }

            // Affiche les options disponibles
            println!("vendre <objet>  (dans votre inventaire)");
            println!("quitter");
            println!("\n➡ Tapez le numéro de l'objet à acheter, ou autre choix :");

            // Attend le choix du joueur
            let mut choix = String::new();
            stdin().read_line(&mut choix).expect("Erreur de lecture");

            // Gère la sortie du commerce
            if choix.trim().eq_ignore_ascii_case("quitter") {
                println!("👋 Bon Affaire");
                break;
            }

            // Gère l'achat d'un objet
            if let Ok(index) = choix.trim().parse::<usize>() {
                if index > 0 && index <= merchant_items.items.len() {
                    let inventory_item = &merchant_items.items[index - 1];
                    let Some(item) = items.iter().find(|i| i.id() == inventory_item.item_id)
                        else { todo!() };
                    
                    // Vérifie si le joueur a assez d'argent
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
        }
    }
}

