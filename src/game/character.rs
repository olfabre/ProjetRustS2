// Gestion des personnages (joueur et PNJ)

use serde::{Serialize, Deserialize};
use crate::game::quest::Quest;
use crate::game::enemy::Enemy;
use crate::game::item::Item;





#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub health: i32,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub level: i32,       // Ajout du niveau du joueur
    pub experience: i32,  // Ajout de l'expérience du joueur
    pub active_quests: Vec<Quest>,
    pub inventory: Vec<Item>,  // Inventaire du joueur
    pub completed_quests: Vec<Quest>,
}

impl Character {
    pub fn new(name: String, strength: i32, agility: i32, intelligence: i32) -> Self {
        Self {
            name,
            health: 100,
            strength,
            agility,
            intelligence,
            level: 1,       // Niveau initial
            experience: 0,   // XP initiale
            inventory: Vec::new(),
            active_quests: Vec::new(),
            completed_quests: Vec::new(),
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

    pub fn accept_quest(&mut self, quest: Quest) {
        println!("📝 Vous avez accepté la quête : {}", quest.name);
        self.active_quests.push(quest);
    }

    pub fn check_quests(&self) {
        println!("📜 Vos quêtes en cours :");
        for (i, quest) in self.active_quests.iter().enumerate() {
            println!("{}. {} - {}", i + 1, quest.name, if quest.completed { "✅ Complétée" } else { "❌ En cours" });
        }
    }

    pub fn attack(&self, target: &mut Enemy) {
        println!("⚔️ {} attaque {} et inflige {} dégâts !", self.name, target.name, self.strength);
        target.health = (target.health - self.strength).max(0);//Empêche d’avoir une valeur négative
    }

    pub fn special_attack(&self, target: &mut Enemy) {
        let damage = self.strength * 2;
        println!("🔥 {} utilise une attaque spéciale et inflige {} dégâts à {} !", self.name, damage, target.name);
        target.health = (target.health - damage).max(0); //Empêche d’avoir une valeur négative
    }

    pub fn add_item(&mut self, item: Item) {
        println!("📦 Vous avez trouvé un objet : {}", item.name);
        self.inventory.push(item);
    }

    pub fn use_item(&mut self) {
        if self.inventory.is_empty() {
            println!("❌ Votre inventaire est vide !");
            return;
        }

        println!("🛍️ Inventaire :");
        for (i, item) in self.inventory.iter().enumerate() {
            println!("{}. {} ({:?})", i + 1, item.name, item.item_type);
        }

        println!("Quel objet voulez-vous utiliser ? (Entrez le numéro ou 0 pour annuler)");
        let choice = crate::game::io_handler::get_user_input().trim().parse::<usize>().unwrap_or(0);

        if choice > 0 && choice <= self.inventory.len() {
            let item = self.inventory.remove(choice - 1);
            item.use_item(self);
        } else {
            println!("❌ Choix invalide.");
        }
    }

    pub fn has_item(&self, item_name: &str) -> bool {
        self.inventory.iter().any(|item| item.name == item_name)
    }

    pub fn check_inventory(&self) {
        println!("🛍️ Inventaire :");
        if self.inventory.is_empty() {
            println!("(Vide)");
        } else {
            for item in &self.inventory {
                println!("📦 {}", item.name);
            }
        }
    }
    pub fn remove_item(&mut self, item_name: &str) {
        self.inventory.retain(|item| item.name != item_name);
    }

    pub fn add_experience(&mut self, xp: i32) {
        println!("🎖️ Vous gagnez {} XP !", xp);
        self.experience += xp;

        // Vérifier si le joueur atteint le niveau suivant
        while self.experience >= self.level * 100 {
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.health += 20;
        self.strength += 2;
        self.agility += 2;
        self.intelligence += 2;
        println!("🔥 Vous montez au niveau {} ! Vos statistiques augmentent.", self.level);
    }
}
