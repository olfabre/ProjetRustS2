<<<<<<< HEAD
// Module de gestion des dialogues et interactions avec les PNJ
// Permet de gérer les conversations, les quêtes et le commerce avec les personnages non-joueurs

=======
// === Imports ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io::stdin;

// Importations de modules internes (modèles de domaine et traits)
use crate::models::entities::character::Character;
use crate::models::entities::item::Item;
use crate::models::entities::pnj::Pnj;
use crate::models::entities::quete::Quete;
use crate::models::traits::money_manager::MoneyManager;

<<<<<<< HEAD
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
=======
// === Structure du dialogue ===
// Représente une conversation interactive avec un PNJ (PNJ)

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue_id: u32,                  // ID unique pour le dialogue
    pub dialogue_steps: Vec<DialogueStep>, // Séquence d'étapes/questions dans le dialogue
}

// === Dialogue Step ===
// Une seule étape dans un dialogue (une question et des options)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueStep {
    pub action: String,     // Action associée (ex. : accepter une quête)
    pub active: bool,       // Indique si cette étape est actuellement active
    pub question: String,   // La question posée au joueur
    pub options: Vec<DialogueOption>, // Liste des choix de réponse
}


// === Dialogue Option ===
// Une réponse possible à une étape de dialogue, avec réaction du PNJ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueOption {
    pub réponse: String,    // Reponse du joueur
    pub réaction: String,   // Réaction du PNJ à la réponse
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
}

// === Dialogue Implementation ===
impl Dialogue {
<<<<<<< HEAD
    // Affiche et gère un dialogue complet avec un PNJ
    // Permet au joueur de choisir des réponses et déclenche les actions associées
    // Gère les quêtes, le commerce et les interactions spéciales
=======

    // === afficher_dialogue ===
    // Gère le dialogue interactif avec le joueur
    // - Affiche les questions du dialogue
    // - Gère l'acceptation/l'achèvement des quêtes
    // - Déclenche le mode marchand si nécessaire
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
    pub fn afficher_dialogue(&mut self, character: &mut Character,
                             quetes: &mut HashMap<u32,Quete>, items: &Vec<Item>, pnj: &mut Pnj) {
        // Vérifie si le PNJ a des dialogues disponibles
        if self.dialogue_steps.is_empty() {
            println!("Ce PNJ n'a rien à dire.");
        } else {
            let mut current_index = 0; // Commence par la première étape du dialogue

<<<<<<< HEAD
            // Parcourt toutes les étapes du dialogue
            while current_index < self.dialogue_steps.len() {
=======
            let mut current_index = 0; // Index de l'étape de dialogue en cours

            while current_index < self.dialogue_steps.len() {

>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                let step = &self.dialogue_steps[current_index];
                let options = self.dialogue_steps[current_index].options.clone();
                let action = self.dialogue_steps[current_index].action.clone();

<<<<<<< HEAD
                // Ignore les étapes inactives
=======
                // Ignorer les étapes inactives
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                if !step.active {
                    current_index += 1;
                    continue;
                }

<<<<<<< HEAD
                // Affiche la question et les options de réponse
=======
                // === Afficher l'étape de dialogue et les options ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                println!("current index: ================> {}", current_index);
                println!("💬 PNJ : \"{}\"", step.question);
                for (i, option) in options.iter().enumerate() {
                    println!("{}. {}", i + 1, option.réponse);
                }

<<<<<<< HEAD
                // Attend le choix du joueur
=======
                // === Lire la saisie utilisateur ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                println!("➡ Tape le numéro de ton choix :");
                let mut choix = String::new();
                stdin().read_line(&mut choix).expect("Erreur de lecture");

<<<<<<< HEAD
                // Traite le choix du joueur
=======
                // === Gérer la réponse de l'utilisateur ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                if let Ok(index) = choix.trim().parse::<usize>() {
                    if index > 0 && index <= step.options.len() {
                        let selected_option = &options[index - 1];
                        println!("💬 PNJ : \"{}\"", selected_option.réaction);

<<<<<<< HEAD
                        // Gère l'acceptation d'une quête
=======
                        // === Accepter quete ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                        if action.starts_with("accepteQuete") && selected_option.réponse == "Accepter Quête" {
                            if let Some(id_str) = action.split(':').nth(1) {
                                if let Ok(id) = id_str.parse::<u32>() {
                                    character.ajouter_quete(id);
                                    println!("🎯 Quête ajoutée : {}!", quetes.get(&id).unwrap().name());
                                    self.dialogue_steps[current_index].active = false;
                                }
                            }
                        }

<<<<<<< HEAD
                        // Gère la complétion d'une quête
=======
                        // === Terminer la quête ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                        if action.starts_with("rendreQuete") && selected_option.réponse == "Completer Quête" {
                            if let Some(id_str) = action.split(':').nth(1) {
                                if let Ok(id) = id_str.parse::<u32>() {
                                    let quete = quetes.get(&id).unwrap();
<<<<<<< HEAD
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
=======

                                    if quete.objectif_type == "collecter" {
                                        character.inventory_mut().remove_item(quete.objectif.collecter.item_id, quete.objectif.collecter.target);
                                        pnj.inventory_mut().add_item(quete.objectif.collecter.item_id, quete.objectif.collecter.target);
                                    }

                                    character.supprimer_quete(id);
                                    character.add_experience(quete.experience);

                                    // === Distribuer les objets de récompense ===
                                    for recompense_item in quete.recompense_items.iter() {
                                        if let Some(item) = items.iter().find(|item| item.id() == *recompense_item) {
                                            character.inventory_mut().add_item(item.id(), 1);
                                            println!("👜 Tu as ramassé '{}'.", item.name());
                                        }
                                    }

                                    // === Distribuer la récompense en argent ===
                                    if quete.recompense_argent > 0 {
                                        println!("🪙 Tu as gagné {} pièces d'money.", quete.recompense_argent);
                                        character.add_money(quete.recompense_argent);
                                    }
                                }
                            }
                        }

                        // === Démarrer le système marchand ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                        if action.starts_with("merchant") && selected_option.réponse == "Commerce" {
                            self.start_merchant(character, items, pnj);
                        }

<<<<<<< HEAD
                        // Termine le dialogue si le joueur choisit de partir
=======
                        // === Quitter le dialogue plus tôt que prévu si le joueur dit au revoir ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                        if selected_option.réponse.starts_with("Au revoir") ||
                            selected_option.réponse.starts_with("Ignorer") ||
                            selected_option.réponse.starts_with("Refuser") {
                            break;
                        }

<<<<<<< HEAD
                        current_index += 1;
=======
                        // Passer à l'étape suivante
                        current_index += 1;

>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
                    } else {
                        println!("❌ Choix invalide !");
                    }
                } else {
                    println!("❌ Entrée invalide !");
                }
            }
        }
    }

<<<<<<< HEAD
    // Démarre l'interface de commerce avec un marchand
    // Permet au joueur d'acheter et vendre des objets
    // Gère l'argent et l'inventaire du joueur et du marchand
=======
    // === start_merchant ===
    // Gère l'achat/la vente d'objets entre le personnage et le PNJ
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
    pub fn start_merchant(&mut self, character: &mut Character, items: &Vec<Item>, pnj: &mut Pnj) {
        loop {
            // Affiche les informations du joueur et du marchand
            println!("\n👤 {} (🪙 {})", character.name(), character.money);
            println!("🛒 Marchand (🪙 {}) : \"Voici mes merchandises.\"", pnj.money);
            let mut merchant_items = pnj.inventory_mut();

<<<<<<< HEAD
            // Affiche la liste des objets disponibles
=======
            // === Lister les articles du marchand ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
            for (i, inventory_item) in merchant_items.items.iter().enumerate() {
                let id = inventory_item.item_id;
                if let Some(item) = items.iter().find(|i| i.id() == id) {
                    println!("{}. {} - 🪙 {} - Qt: {}", i + 1, item.name(), item.value, inventory_item.quantity);
                } else {
                    println!("{}. Objet inconnu (ID: {})", i + 1, id);
                }
            }

<<<<<<< HEAD
            // Affiche les options disponibles
            println!("vendre <objet>  (dans votre inventaire)");
=======
            println!("vendre <objet>, inventaire (afficher inventaire)");
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
            println!("quitter");
            println!("\n➡ Tapez le numéro de l'objet à acheter, ou autre choix :");

            // Attend le choix du joueur
            let mut choix = String::new();
            stdin().read_line(&mut choix).expect("Erreur de lecture");

<<<<<<< HEAD
            // Gère la sortie du commerce
=======
            // === Quitter le mode marchand ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
            if choix.trim().eq_ignore_ascii_case("quitter") {
                println!("👋 Bon Affaire");
                break;
            }

<<<<<<< HEAD
            // Gère l'achat d'un objet
=======
            // === Achat d'articles ===
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
            if let Ok(index) = choix.trim().parse::<usize>() {
                if index > 0 && index <= merchant_items.items.len() {
                    let inventory_item = &merchant_items.items[index - 1];
                    let Some(item) = items.iter().find(|i| i.id() == inventory_item.item_id)
<<<<<<< HEAD
                        else { todo!() };
                    
                    // Vérifie si le joueur a assez d'argent
=======
                    else { todo!() };

>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
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
<<<<<<< HEAD
=======
            }

            // === Vente d'articles ===
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

            // === Afficher l'inventaire ===
            } else if choix.starts_with("inventaire") {
                character.afficher_inventaire(items);
            } else {
                println!("❌ Entrée invalide !");
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
            }
        }
    }
}
