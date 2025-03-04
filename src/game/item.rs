use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub effect: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ItemType {
    Potion,    // Soigne le joueur
    Weapon,    // Augmente l'attaque
    Armor,     // Augmente la défense
}

impl Item {
    pub fn new(name: &str, item_type: ItemType, effect: i32) -> Self {
        Self {
            name: name.to_string(),
            item_type,
            effect,
        }
    }

    pub fn use_item(&self, player: &mut crate::game::character::Character) {
        match self.item_type {
            ItemType::Potion => {
                player.health += self.effect;
                println!("🧪 Vous avez utilisé {} et récupéré {} points de vie !", self.name, self.effect);
            }
            ItemType::Weapon => {
                player.strength += self.effect;
                println!("⚔️ Vous avez équipé {} et gagné {} points de force !", self.name, self.effect);
            }
            ItemType::Armor => {
                player.agility += self.effect;
                println!("🛡️ Vous avez équipé {} et gagné {} points d'agilité !", self.name, self.effect);
            }
        }
    }
}
