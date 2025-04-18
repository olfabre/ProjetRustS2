use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde::de::Visitor;
use crate::models::traits::{Movable, Descriptible};
use crate::models::{entities::room::Room, entities::item::Item};
use crate::models::entities::entity::Entity;
use crate::models::entities::quete::Quete;
use crate::models::entities::vivant::Vivant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    vivant: Vivant,

    pub position: usize,
    pub level: i32,       // Ajout du niveau du joueur
    pub experience: i32,  // Ajout de l'expérience du joueur
    pub quests: Vec<u32>


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

                    // Affiche le nom et la description de la salle dans laquelle on vient d’entrer
                    println!("✅ {} est maintenant dans : {}", self.name(), next_room.name());
                    println!("📖 Description : {}", next_room.description());
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
    pub fn prendre_objet(&mut self, objet_nom: &str, rooms: &mut [Room], items: &[Item]) {
        // On convertit le nom pour ignorer la casse lors de la comparaison
        let objet_nom = objet_nom.to_lowercase();
        let current_room = &mut rooms[self.position];

        // On cherche l'objet dans la salle actuelle
        if let Some((index, item_id)) = current_room
            .items
            .iter()
            .enumerate()
            .find_map(|(i, id)| {
                items.iter().find(|item| item.id() == *id && item.name().to_lowercase() == objet_nom).map(|_| (i, *id))
            })
        {
            // On récupère l'objet depuis la liste globale
            if let Some(item) = items.iter().find(|item| item.id() == item_id) {
                self.inventory_mut().push(item.clone()); // On l'ajoute à l'inventaire du personnage
                current_room.items.remove(index);    // Et on le retire de la salle
                println!("👜 Tu as ramassé '{}'.", item.name());
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
        // Mise en minuscule du nom pour comparer sans tenir compte de la casse
        let objet_nom = objet_nom.to_lowercase();

        // On cherche l'objet dans l'inventaire du personnage
        if let Some(index) = self.inventory_mut().iter().position(|i| i.name().to_lowercase() == objet_nom) {
            let objet = &self.inventory_mut()[index].clone();

            // Vérifie si l'objet peut être utilisé
            if !objet.usable {
                println!("⚠️ L'objet '{}' ne peut pas être utilisé directement.", objet.name());
                return;
            }

            // On agit en fonction de l'effet de l'objet
            match objet.effect() {
                // Potion de soin : restaure 20 PV
                ("heal_20") => {
                    self.set_health(self.health() + 20);
                    if self.health() > 100 { self.set_health(100); } // PV max = 100
                    println!("💊 Tu as utilisé {}. PV restaurés à {}.", objet.name(), self.health());
                    self.inventory_mut().remove(index); // Objet consommé
                }

                // Bonus d'attaque (ex. Épée de fer)
                ("attack_bonus_5") => {
                    self.set_health(self.strength() + 5);
                    println!("⚔️ Tu te sens plus fort ! Bonus de force activé (+5).");
                    // Pas supprimé si c’est un objet permanent
                }

                // Parchemin mystérieux : effet narratif
                ("Dévoile un secret ancien.") => {
                    println!("📜 Le parchemin révèle une énigme cachée : 'Cherche là où les ombres dansent...'");
                    self.inventory_mut().remove(index); // Consommé après usage
                }

                // Clé du trésor : déverrouille la salle actuelle si elle est verrouillée
                ("Déverrouille la salle du trésor.") => {
                    let current_room = &mut rooms[self.position];
                    if current_room.locked.unwrap_or(false) {
                        current_room.locked = Some(false); // On déverrouille
                        println!("🔓 Tu as utilisé la clé. La salle '{}' est maintenant déverrouillée !", current_room.name());
                        self.inventory_mut().remove(index); // Clé supprimée après usage
                    } else {
                        println!("ℹ️ Il n'y a rien à déverrouiller ici.");
                    }
                }

                // Pour tout autre effet générique
                (effet) => {
                    println!("✨ Tu utilises '{}', effet : {}", objet.name(), effet);
                    // Tu peux personnaliser ici selon tes besoins
                }


            }
        } else {
            // L'objet n'est pas dans l'inventaire du personnage
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
    pub fn afficher_inventaire(&mut self) {
        println!("🎒 Inventaire :");
        if self.inventory_mut().is_empty() {
            println!("(vide)");
        } else {
            for item in self.inventory_mut() {
                println!("- {} : {} (Effet : {})", item.name(), item.description(), item.effect());
            }
        }
    }

    pub fn ajouter_quete(&mut self, id: u32) {

        self.quests.push(id);
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

    pub fn inventory_mut(&mut self) -> &mut Vec<Item> {
        self.vivant.inventory_mut()
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





}
