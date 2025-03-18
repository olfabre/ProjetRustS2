use crate::models::traits::{Descriptible, Movable};
use crate::models::{item::Item, room::Room};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub id: u32,
    pub name: String,
    pub health: i32,
    pub strength: i32,
    pub intelligence: i32,
    pub position: usize,
    pub inventory: Vec<Item>,
}

impl Descriptible for Character {
    fn get_description(&self) -> String {
        format!(
            "{} (Santé: {}, Force: {}, Intelligence: {})",
            self.name, self.health, self.strength, self.intelligence
        )
    }
}

impl Movable for Character {
    //Le personnages doit se déplacer avec une direction
    fn move_to(&mut self, direction: &str, rooms: &[Room]) {
        if let Some(current_room) = rooms.get(self.position) {
            if let Some(&new_position) = current_room.exits.get(direction) {
                self.position = new_position;
                println!(
                    "{} se déplace vers {}.",
                    self.name, rooms[new_position].name
                );
            } else {
                println!("❌ Pas de passage dans cette direction !");
            }
        } else {
            println!("❌ Erreur : Salle inconnue.");
        }
    }
}

impl Character {
    //on ne doit pas avoir trop rooms
    pub fn prendre_objet(&mut self, objet_nom: &str, rooms: &mut [Room], items: &[Item]) {
        if let Some(room) = rooms.get_mut(self.position) {
            if let Some(&item_id) = room.items.iter().find(|&&id| {
                items.iter().any(|item| {
                    item.id == id && item.name.to_lowercase() == objet_nom.to_lowercase()
                })
            }) {
                if let Some(item) = items.iter().find(|i| i.id == item_id) {
                    room.items.retain(|&id| id != item_id); // ✅ Supprimer l'objet de la salle
                    self.inventory.push(item.clone()); // ✅ Ajouter l'objet dans l'inventaire
                    println!("🎒 {} a pris l'objet : {}", self.name, item.name);
                }
            } else {
                println!("❌ Objet non trouvé dans cette salle !");
            }
        }
    }

    pub fn utiliser_objet(&mut self, objet_nom: &str) {
        let objet_nom = objet_nom.to_lowercase();

        if let Some(index) = self
            .inventory
            .iter()
            .position(|item| item.name.to_lowercase() == objet_nom)
        {
            let item = self.inventory.remove(index);

            match item.name.as_str() {
                "Torche" => {
                    println!(
                        "🔥 {} allume la torche. La salle est maintenant éclairée !",
                        self.name
                    );
                }
                "Potion de soin" => {
                    self.health += 10;
                    println!(
                        "🧪 {} boit une potion et récupère 10 points de vie. (Santé : {})",
                        self.name, self.health
                    );
                }
                "Gemme enchantée" => {
                    println!("💎 {} sent une force mystique l'entourer.", self.name);
                }
                _ => {
                    println!("❌ Cet objet ne peut pas être utilisé.");
                }
            }
        } else {
            println!("❌ Tu ne possèdes pas cet objet dans ton inventaire !");
        }
    }

    //L'inventaire de l'objet pas de la character(&self)
    pub fn afficher_inventaire(&self) {
        println!("🎒 Inventaire :");
        if self.inventory.is_empty() {
            println!("(vide)");
        } else {
            for item in &self.inventory {
                println!(
                    "- {} : {} (Effet : {})",
                    item.name,
                    item.description,
                    item.effect.as_deref().unwrap_or("Aucun")
                );
            }
        }
    }
}
