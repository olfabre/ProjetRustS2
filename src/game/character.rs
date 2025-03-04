use serde::{Serialize, Deserialize};
use crate::game::quest::Quest;
use crate::game::enemy::Enemy;
use crate::game::item::Item;
use crate::game::data_loader::load_game_data;

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub health: i32,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub level: i32,
    pub experience: i32,
    pub inventory: Vec<Item>,
    pub active_quests: Vec<Quest>,
    pub completed_quests: Vec<Quest>,
}

impl Character {
    pub fn load_from_json() -> Self {
        let data = load_game_data();
        data.characters.into_iter().next().expect("Aucun personnage trouvé")
    }
}



    pub fn display(&self) {
        println!("👤 Nom : {}", self.name);
        println!("❤️ Santé : {}", self.health);
        println!("💪 Force : {}", self.strength);
        println!("⚡ Agilité : {}", self.agility);
        println!("🧠 Intelligence : {}", self.intelligence);
        println!("🎖️ Niveau : {}", self.level);
        println!("🔹 XP : {}/{}", self.experience, self.level * 100);
    }


