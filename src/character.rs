use serde::{Serialize, Deserialize};
use crate::data_loader::{load_game_data, Character};
use crate::item::Item;


impl Character {
    pub fn load_from_json() -> Self {
        let data = load_game_data();
        data.characters.into_iter().next().expect("Aucun personnage trouvé")
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

    /// Vérifie les quêtes en cours du joueur
    pub fn check_quests(&self) {
        if self.active_quests.is_empty() {
            println!("📜 Vous n'avez aucune quête en cours.");
        } else {
            println!("📜 Quêtes en cours :");
            for (i, quest) in self.active_quests.iter().enumerate() {
                println!("{}. {} - {}", i + 1, quest.name, if quest.completed { "✅ Complétée" } else { "❌ En cours" });
            }
        }
    }

    /// Affiche l'inventaire du joueur
    pub fn check_inventory(&self) {
        if self.inventory.is_empty() {
            println!("🛍️ Votre inventaire est vide.");
        } else {
            println!("🛍️ Inventaire :");
            for (i, item) in self.inventory.iter().enumerate() {
                println!("{}. {} ({:?})", i + 1, item.name, item.item_type);
            }
        }
    }
}
