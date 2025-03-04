use crate::game::quest::Quest;
use crate::game::data_loader::load_game_data;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Npc {
    pub name: String,
    pub role: String,
    pub dialogue: Vec<String>,
    pub quests: Vec<Quest>,
}

impl Npc {
    /// Charge les PNJ depuis `game_data.json`
    pub fn load_from_json() -> Vec<Self> {
        let data = load_game_data();
        let npcs: Vec<Npc> = data.world.zones.iter().flat_map(|zone| zone.npcs.clone()).collect();
        if npcs.is_empty() {
            println!("⚠️ Aucun PNJ trouvé dans le jeu.");
        }
        npcs
    }

    pub fn talk(&self, player: &mut crate::game::character::Character) {
        println!("👤 {} : \"{}\"", self.name, self.dialogue[0]);

        if let Some(quest) = self.quests.first() {
            println!("🔹 {} a une quête pour vous : {}", self.name, quest.name);
            println!("   📜 Description : {}", quest.description);
            println!("   🎁 Récompense : {}", quest.reward);
            println!("Voulez-vous accepter cette quête ? (oui/non)");

            let response = crate::game::io_handler::get_user_input();
            if response.trim().to_lowercase() == "oui" {
                player.accept_quest(quest.clone());
            }
        }
    }
}
