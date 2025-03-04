//Système de quêtes

use serde::{Serialize, Deserialize};
use crate::game::item::Item;


#[derive(Serialize, Deserialize, Clone)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub reward: String,
    pub completed: bool,
}

impl Quest {
    pub fn new(name: &str, description: &str, reward: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            reward: reward.to_string(),
            completed: false,
        }
    }

    pub fn complete(&mut self, player: &mut crate::game::character::Character) {
        self.completed = true;
        println!("✅ Quête complétée : {} ! Récompense obtenue : {}", self.name, self.reward);

        let reward_item = Item::new(&self.reward, crate::game::item::ItemType::Potion, 0);
        player.add_item(reward_item);
        player.add_experience(100); // Récompense XP pour une quête complétée
    }
}
