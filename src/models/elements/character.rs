use serde::{Deserialize, Serialize};
use crate::models::traits::{Descriptible, Movable};
use crate::models::room::Room;
use crate::models::elements::item::Item;
use crate::models::elements::vivant::Vivant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    vivant: Vivant,
    position: usize,

}

impl Descriptible for Character {
    fn get_description(&self) -> String {
        format!(
            "{} (Santé: {}, Force: {}, Intelligence: {})",
            self.vivant.name(), self.vivant.health(), self.vivant.strength(), self.vivant.intelligence()
        )
    }
}

impl Movable for Character {
    //Le personnages doit se déplacer avec une direction
    fn move_to(&mut self, direction: &str, rooms: &[Room]) {
        if let Some(current_room) = rooms.get(self.position) {
            if let Some(&new_position) = current_room.exits.get(direction) {
                self.position = new_position;
                println!("{} se déplace vers {}.", self.vivant.name(), rooms[new_position].name);
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
                items.iter().any(|item| item.id() == id && item.name().to_lowercase() == objet_nom.to_lowercase())
            }) {
                if let Some(item) = items.iter().find(|i| i.id() == item_id) {
                    room.items.retain(|&id| id != item_id);  // ✅ Supprimer l'objet de la salle
                    self.inventory().push(item.clone());       // ✅ Ajouter l'objet dans l'inventaire
                    println!("🎒 {} a pris l'objet : {}", self.vivant.name(), item.name());
                }
            } else {
                println!("❌ Objet non trouvé dans cette salle !");
            }
        }
    }

    pub fn utiliser_objet(&mut self, objet_nom: &str) {
        let objet_nom = objet_nom.to_lowercase();

        if let Some(index) = self.vivant.inventory().iter().position(|item| item.name().to_lowercase() == objet_nom) {
            let item = self.inventory().remove(index);

            match item.name() {
                "Torche" => {
                    println!("🔥 {} allume la torche. La salle est maintenant éclairée !", self.vivant.name());
                }
                "Potion de soin" => {
                    self.vivant.set_health(self.vivant.health() + 10);
                    println!("🧪 {} boit une potion et récupère 10 points de vie. (Santé : {})", self.vivant.name(), self.vivant.health());
                }
                "Gemme enchantée" => {
                    println!("💎 {} sent une force mystique l'entourer.", self.vivant.name());
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
    pub fn afficher_inventaire(&mut self) {
        println!("🎒 Inventaire :");
        if self.inventory().is_empty() {
            println!("(vide)");
        } else {
            for item in self.inventory() {
                println!("- {} : {} (Effet : {})", item.name(), item.description(), item.effect().as_deref().unwrap_or("Aucun"));
            }
        }
    }

    pub fn id(&self) -> u32 {
        self.vivant.id()
    }

    pub fn name(&self) -> &str {
        &self.vivant.name()
    }

    pub fn description(&self) -> &str {
        &self.vivant.description()
    }

    pub fn health(&self) -> u32 {
        self.vivant.health()
    }

    pub fn strength(&self) -> u32 {
        self.vivant.strength()
    }

    pub fn intelligence(&self) -> u32 {
        self.vivant.intelligence()
    }

    // pub fn inventory(&self) -> &Vec<Item> {
    //     &self.vivant.inventory()
    // }

    pub fn inventory(&mut self) -> &mut Vec<Item> {
        self.vivant.inventory()
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn set_name(&mut self, name: &str) {
        self.vivant.set_name(name);
    }

    pub fn set_id(&mut self, id: u32) {
        self.vivant.set_id(id);
    }

    pub fn set_description(&mut self, desc: &str) {
        self.vivant.set_description(desc);
    }

    pub fn set_health(&mut self, health: u32) {
        self.vivant.set_health(health);
    }

    pub fn set_strength(&mut self, strength: u32) {
        self.vivant.set_health(strength);
    }

    pub fn set_intelligence(&mut self, intelligence: u32) {
        self.vivant.set_intelligence(intelligence);
    }

    pub fn set_inventory(&mut self, inventory: Vec<Item>) {
        self.vivant.set_inventory(inventory);
    }


}
