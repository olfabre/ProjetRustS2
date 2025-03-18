use serde::{Deserialize, Serialize};
use std::io::stdin;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue_id: u32,
    pub pnj_id: u32, // Associe un dialogue à un PNJ
    pub dialogues: Vec<DialogueStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogueStep {
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
    pub fn afficher_dialogue(&self) {
        if let Some(first_step) = self.dialogues.first() {
            self.afficher_et_gérer_choix(first_step);
        } else {
            println!("💬 Ce PNJ n'a rien à dire.");
        }
    }

    /// 🔄 Gère un échange de dialogue en affichant les options et en traitant la réponse
    fn afficher_et_gérer_choix(&self, step: &DialogueStep) {
        println!("💬 PNJ : \"{}\"", step.question);

        for (i, option) in step.options.iter().enumerate() {
            println!("{}. {}", i + 1, option.réponse);
        }

        println!("➡ Tape le numéro de ton choix :");
        let mut choix = String::new();
        stdin().read_line(&mut choix).expect("Erreur de lecture");

        if let Ok(index) = choix.trim().parse::<usize>() {
            if index > 0 && index <= step.options.len() {
                println!("💬 PNJ : \"{}\"", step.options[index - 1].réaction);
            } else {
                println!("❌ Choix invalide !");
            }
        } else {
            println!("❌ Entrée invalide !");
        }
    }
}
