use crate::game::quest::Quest;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Npc {
    pub name: String,
    pub role: String,
    pub dialogue: Vec<String>,
    pub quests: Vec<Quest>,
}

impl Npc {
    pub fn talk(&self, player: &mut crate::game::character::Character) {
        println!("{} : {}", self.name, self.dialogue[0]);

        if self.name == "Gérard" {
            // 🔥 Vérifier si la quête est DÉJÀ terminée AVANT toute action
            if player.completed_quests.iter().any(|q| q.name == "Trouver l'amulette perdue") {
                println!("✅ Vous avez déjà complété cette quête !");
                return;
            }

            // 🔥 Vérifier si la quête est en cours mais PAS encore terminée
            if let Some(quest_index) = player.active_quests.iter().position(|q| q.name == "Trouver l'amulette perdue") {
                if player.has_item("Amulette perdue") {
                    // ✅ La quête est complétée -> Récompense + marquer comme terminée
                    let mut quest = player.active_quests.remove(quest_index);
                    quest.complete(player);
                    player.completed_quests.push(quest.clone());  // ✅ Marquer comme terminée
                    player.remove_item("Amulette perdue");
                    println!("🎉 Quête complétée : {} !", quest.name);
                    println!("🔑 Vous avez reçu la Clé dorée !");
                } else {
                    println!("📜 Vous devez encore trouver l'amulette perdue !");
                }
                return;  // ✅ Empêche Gérard de reproposer la quête
            }

            // 🔥 Si la quête n'est PAS acceptée, alors on peut la proposer
            if !player.active_quests.iter().any(|q| q.name == "Trouver l'amulette perdue") {
                if let Some(quest) = self.quests.first() {
                    println!("🔹 {} a une quête pour vous : {}", self.name, quest.name);
                    println!("   📜 Description : {}", quest.description);
                    println!("   🎁 Récompense : {}", quest.reward);
                    println!("Voulez-vous accepter cette quête ? (oui/non)");

                    let response = crate::game::io_handler::get_user_input();
                    if response.trim().to_lowercase() == "oui" {
                        player.accept_quest(quest.clone());
                        println!("📝 Vous avez accepté la quête : {}", quest.name);
                    }
                }
            }
        }
    }
}