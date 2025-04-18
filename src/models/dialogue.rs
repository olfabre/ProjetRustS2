use serde::{Deserialize, Serialize};
use std::io::stdin;
use crate::models::entities::character::Character;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue_id: u32,
    pub dialogues: Vec<DialogueStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogueStep {
    pub action: String,
    pub active: bool,
    pub question: String,
    pub options: Vec<DialogueOption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogueOption {
    pub réponse: String,
    pub réaction: String,
}

impl Dialogue {
    /// Affiche un dialogue et permet au joueur de choisir une réponse
    pub fn afficher_dialogue(&self, character: &mut Character) {
        if !self.dialogues.is_empty() {
            self.afficher_et_gérer_choix(character);
        } else {
            println!("💬 Ce PNJ n'a rien à dire.");
        }
    }

    /// 🔄 Gère un échange de dialogue en affichant les options et en traitant la réponse
    /// 🔄 Gère un échange de dialogue en affichant les options et en traitant la réponse
    fn afficher_et_gérer_choix(&self, character: &mut Character) {
        let mut current_index = 0; // Start at the first dialogue step

        while current_index < self.dialogues.len() {
            let step = &self.dialogues[current_index];

            // ⏩ Skip inactive steps
            if !step.active {
                current_index += 1;
                continue;
            }

            println!("💬 PNJ : \"{}\"", step.question);

            for (i, option) in step.options.iter().enumerate() {
                println!("{}. {}", i + 1, option.réponse);
            }

            println!("➡ Tape le numéro de ton choix :");
            let mut choix = String::new();
            stdin().read_line(&mut choix).expect("Erreur de lecture");

            if let Ok(index) = choix.trim().parse::<usize>() {
                if index > 0 && index <= step.options.len() {
                    let selected_option = &step.options[index - 1];

                    // Afficher la réaction
                    println!("💬 PNJ : \"{}\"", selected_option.réaction);

                    // 🛠️ Vérifier si l'action commence par "accepteQuete"
                    if step.action.starts_with("accepteQuete") && selected_option.réponse == "Accepter Quête" {
                        // Extraire l'ID de la quête (si format "accepteQuete:42")
                        if let Some(id_str) = step.action.split(':').nth(1) {
                            if let Ok(id) = id_str.parse::<u32>() {
                                self.accepter_quete(id, character);
                            }
                        }
                    }

                    // 🔍 Rechercher si la réaction correspond à une question existante
                    if let Some((new_index, _)) = self.dialogues
                        .iter()
                        .enumerate()
                        .find(|(_, d)| d.question == selected_option.réaction)
                    {
                        current_index = new_index; // ✅ Mettre à jour l'index
                        continue; // 🔄 Passer à la prochaine question
                    }

                    // 🛑 Fin du dialogue si aucune nouvelle question ne correspond
                    break;
                } else {
                    println!("❌ Choix invalide !");
                }
            } else {
                println!("❌ Entrée invalide !");
            }
        }
    }

    /// 🎯 Accepter une quête
    pub fn accepter_quete(&self, id: u32, character: &mut Character) {
        println!("🗺️ Quête {} acceptée !", id);

        character.ajouter_quete(id);
        // Ajouter la quête au journal du joueur ici
    }
}
