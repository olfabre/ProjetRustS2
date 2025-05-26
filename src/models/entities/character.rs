use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde::de::Visitor;
use crate::models::traits::{Movable, Descriptible};
use crate::models::{entities::room::Room, entities::item::Item};
use crate::models::dialogue::Dialogue;
use crate::models::entities::entity::Entity;
use crate::models::entities::inventory::Inventory;
use crate::models::entities::quete::Quete;
use crate::models::entities::vivant::Vivant;
use crate::models::tracker::Tracker;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    vivant: Vivant,

    pub position: usize, // Le room_id du personnage a present
    pub level: i32,       // Ajout du niveau du joueur
    pub experience: i32,  // Ajout de l'expérience du joueur
    pub money: i32,
    pub quests: Vec<u32> // references to quete


}

impl Descriptible for Character {
    fn get_description(&self) -> String {
        format!(
            "{} (Santé: {}, Force: {}, Intelligence: {})",
            self.name(), self.health(), self.strength(), self.intelligence()
        )
    }
}

impl Movable for Character {

    /// Méthode simple pour déplacer le joueur sans vérification
    fn move_to_position(&mut self, new_position: usize) {
        self.position = new_position;
    }

    fn get_position(&self) {
        todo!()
    }
}

impl Character {

    /// Tente de déplacer le joueur dans la direction donnée,
    /// en vérifiant la direction et si la salle n’est pas verrouillée.
    pub fn try_move(&mut self, direction: &str, rooms: &[Room]) {
        // Récupère la salle actuelle à partir de la position du personnage
        let current_room = &rooms[self.position];

        // Vérifie si une sortie existe dans la direction demandée
        if let Some(&next_room_id) = current_room.exits.get(direction) {
            // Récupère la salle vers laquelle on veut se déplacer
            if let Some(next_room) = rooms.get(next_room_id) {
                // Vérifie si la salle est verrouillée (locked = true)
                if next_room.locked.unwrap_or(false) {
                    // Si oui, empêche le déplacement et affiche un message d'information
                    println!("🚪 La salle '{}' est verrouillée. Tu as besoin d'une clé ou d'une action spéciale pour entrer.", next_room.name());
                } else {
                    // Sinon, met à jour la position du personnage vers la nouvelle salle
                    self.position = next_room_id;

                    // Ça, c'est déja affiché sur le boucle, on doit pas le repeter
                    // Affiche le nom et la description de la salle dans laquelle on vient d’entrer
                    // println!("✅ {} est maintenant dans : {}", self.name(), next_room.name());
                    // println!("📖 Description : {}", next_room.description());
                }
            } else {
                // Si la salle n’existe pas (ID invalide), affiche un message d’erreur
                println!("❌ Salle inconnue.");
            }
        } else {
            // Si la direction n’existe pas depuis cette salle, affiche un message d’erreur
            println!("❌ Direction invalide.");
        }
    }

    //on ne doit pas avoir trop rooms
    /*pub fn prendre_objet(&mut self, objet_nom: &str, rooms: &mut [Room], items: &[Item]) {
        if let Some(room) = rooms.get_mut(self.position) {
            if let Some(&item_id) = room.items.iter().find(|&&id| {
                items.iter().any(|item| item.id == id && item.name.to_lowercase() == objet_nom.to_lowercase())
            }) {
                if let Some(item) = items.iter().find(|i| i.id == item_id) {
                    room.items.retain(|&id| id != item_id);  // ✅ Supprimer l'objet de la salle
                    self.inventory.push(item.clone());       // ✅ Ajouter l'objet dans l'inventaire
                    println!("🎒 {} a pris l'objet : {}", self.name, item.name);
                }
            } else {
                println!("❌ Objet non trouvé dans cette salle !");
            }
        }
    }*/
    pub fn prendre_objet(&mut self, objet_nom: &str,
                         rooms: &mut [Room], items: &[Item],
                         quetes: &mut HashMap<u32, Quete>,
                         dialogues: &mut Vec<Dialogue>) {
        // On convertit le nom pour ignorer la casse lors de la comparaison
        let objet_nom = objet_nom.to_lowercase();
        let current_room = &mut rooms[self.position];


        // On cherche l'objet dans la salle actuelle par nom (case-insensitive)
        if let Some((index, item_id)) = current_room
            .items
            .iter()
            .enumerate()
            .find_map(|(i, id)| {
                items
                    .iter()
                    .find(|item| item.id() == *id && item.name().to_lowercase() == objet_nom)
                    .map(|_| (i, *id))
            })
        {
            if let Some(item) = items.iter().find(|item| item.id() == item_id) {
                let ajouté = self.vivant.inventory.add_item(item.id(), 1);

                if ajouté {
                    current_room.items.remove(index); // Retirer l'objet de la salle
                    println!("👜 Tu as ramassé '{}'.", item.name());
                    Character::track_item(item_id, self, quetes, dialogues);
                } else {
                    println!("❌ Inventaire plein, impossible de ramasser '{}'.", item.name());
                }
            }
        } else {
            println!("❌ Aucun objet nommé '{}' trouvé ici.", objet_nom);
        }
    }


    /*pub fn utiliser_objet(&mut self, objet_nom: &str) {
        let objet_nom = objet_nom.to_lowercase();

        if let Some(index) = self.inventory.iter().position(|item| item.name.to_lowercase() == objet_nom) {
            let item = self.inventory.remove(index);

            match item.name.as_str() {
                "Torche" => {
                    println!("🔥 {} allume la torche. La salle est maintenant éclairée !", self.name);
                }
                "Potion de soin" => {
                    self.health += 10;
                    println!("🧪 {} boit une potion et récupère 10 points de vie. (Santé : {})", self.name, self.health);
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
    }*/

    /// Permet au personnage d'utiliser un objet de son inventaire
    pub fn utiliser_objet(&mut self, objet_nom: &str, rooms: &mut [Room], items: &[Item]) {
        let objet_nom = objet_nom.to_lowercase();

        // Recherche de l'objet dans l'inventaire
        if let Some(index) = self.inventory_mut().items.iter().position(|inv_item| {
            items.iter().any(|item| item.id() == inv_item.item_id && item.name().to_lowercase() == objet_nom)
        }) {
            let inv_item = self.inventory_mut().items[index].clone();

            // Récupération du vrai Item
            if let Some(item) = items.iter().find(|i| i.id() == inv_item.item_id) {
                if !item.usable {
                    println!("❌ {} ne peut pas être utilisé.", item.name());
                    return;
                }

                match item.effect() {
                    "heal_20" => {
                        self.set_health((self.health() + 20).min(100));
                        println!("💊 Tu as utilisé {}. PV restaurés à {}.", item.name(), self.health());
                        self.inventory_mut().remove_item(item.id(), 1);
                    }

                    "attack_bonus_5" => {
                        self.set_strength(self.strength() + 5);
                        println!("⚔️ Tu te sens plus fort ! Bonus de force activé (+5).");
                        // Pas consommé ici
                    }

                    "Dévoile un secret ancien." => {
                        println!("📜 Le parchemin révèle une énigme : 'Cherche là où les ombres dansent...'");
                        self.inventory_mut().remove_item(item.id(), 1);
                    }

                    "Déverrouille la salle du trésor." => {
                        let current_room = &mut rooms[self.position];
                        if current_room.locked.unwrap_or(false) {
                            current_room.locked = Some(false);
                            println!("🔓 Tu as utilisé la clé. La salle '{}' est maintenant déverrouillée !", current_room.name());
                            self.inventory_mut().remove_item(item.id(), 1);
                        } else {
                            println!("ℹ️ Il n'y a rien à déverrouiller ici.");
                        }
                    }

                    effet => {
                        println!("✨ Tu utilises '{}', effet : {}", item.name(), effet);
                    }
                }
            } else {
                println!("❌ Impossible d'identifier cet objet dans la base de données.");
            }
        } else {
            println!("❌ Tu ne possèdes pas cet objet.");
        }
    }




    pub fn level_up(&mut self) {
        self.level += 1;
        self.set_health(self.health() + 20);
        self.set_strength(self.strength() + 2);
        self.set_intelligence(self.intelligence() + 2);
        println!("🔥 Vous montez au niveau {} ! Vos statistiques augmentent.", self.level);
    }
    pub fn add_experience(&mut self, xp: i32) {
        println!("🎖️ Vous gagnez {} XP !", xp);
        self.experience += xp;

        // Vérifier si le joueur atteint le niveau suivant
        while self.experience >= self.level * 100 {
            self.level_up();
        }
    }



    //L'inventaire de l'objet pas de la character(&self)
    //L'inventaire de l'objet pas de la character(&self)
    pub fn afficher_inventaire(&self, items: &[Item]) {
        println!("\n🎒 Inventaire :");

        if self.vivant.inventory.items.is_empty() {
            println!("(vide)");
            return;
        }

        for inv_item in &self.vivant.inventory.items {
            if let Some(full_item) = items.iter().find(|i| i.id() == inv_item.item_id) {
                println!(
                    "- {} x{} ({})",
                    full_item.name(),
                    inv_item.quantity,
                    full_item.effect()
                );
            } else {
                println!(
                    "- Objet inconnu (id {}) x{}",
                    inv_item.item_id,
                    inv_item.quantity
                );
            }
        }
    }

    pub fn ajouter_quete(&mut self, id: u32) {
        self.quests.push(id);
    }

    pub fn supprimer_quete(&mut self, id: u32) {
        self.quests.retain(|&q| q != id);
    }


    pub fn id(&self) -> u32 {
        self.vivant.id()
    }

    pub fn name(&self) -> &str {
        self.vivant.name()
    }

    pub fn health(&self) -> i32 {
        self.vivant.health()
    }

    pub fn strength(&self) -> i32 {
        self.vivant.strength()
    }

    pub fn intelligence(&self) -> i32 {
        self.vivant.intelligence()
    }

    pub fn set_health(&mut self, health: i32) {
        self.vivant.set_health(health);
    }

    pub fn set_strength(&mut self, strength: i32) {
        self.vivant.set_strength(strength);
    }

    pub fn set_intelligence(&mut self, intelligence: i32) {
        self.vivant.set_intelligence(intelligence);
    }

    pub fn inventory_mut(&mut self) -> &mut Inventory {
        self.vivant.inventory_mut()
    }

    pub fn add_inventory(&mut self, item_id: u32) {

    }

    pub fn get_active_quests(&self, all_quests: &HashMap<u32, Quete>) -> Vec<String> {
        // Create a vector to store the names of matching quests.
        let mut quest_titles: Vec<String> = vec![];

        // Iterate over each quest ID in the character's quests list.
        for &quest_id_from_char in &self.quests {
            // Find the quest in the all_quests list that matches the ID.
            if let Some(quest_found) = all_quests.get(&quest_id_from_char) {
                // Clone the name and push it to the quest_titles vector.
                quest_titles.push(quest_found.name().to_string());
            }
        }

        // Return the collected quest titles.
        quest_titles
    }

    pub fn quests(&self) -> &Vec<u32> {
        &self.quests
    }

    pub fn add_argent(&mut self, quantité: i32) {
        self.money += quantité;
    }

}

impl Tracker for Character {

}