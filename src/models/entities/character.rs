use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde::de::Visitor;

use crate::models::traits::descriptible::Descriptible;
use crate::models::{entities::room::Room, entities::item::Item};
use crate::models::dialogue::Dialogue;
use crate::models::entities::Enemy::Enemy;
use crate::models::entities::entity::Entity;
use crate::models::entities::inventory::Inventory;
use crate::models::entities::quete::Quete;
use crate::models::entities::vivant::Vivant;
use crate::models::tracker::Tracker;
use crate::models::traits::combattant::{CombatResult, Combattant};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub(crate) vivant: Vivant,

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


impl Character {

    /// Tente de déplacer le joueur dans la direction donnée,
    /// en vérifiant la direction et si la salle n'est pas verrouillée.
    pub fn try_move(&mut self, direction: &str, rooms: &mut [Room]) {
        // Récupère la salle actuelle à partir de la position du personnage
        let current_room = &rooms[self.position];

        // Vérifie si une sortie existe dans la direction demandée
        if let Some(&next_room_id) = current_room.exits.get(direction) {
            // Recherche la salle cible par son id (et non par son index !)
            if let Some(next_room) = rooms.iter_mut().find(|r| r.id() == next_room_id as u32) {
                // println!("DEBUG: locked = {:?}", next_room.locked); // Affichage debug
                if next_room.locked.unwrap_or(true) {
                    // On tente d'ouvrir la porte (lancer de dés 421)
                    if next_room.tenter_ouverture() {
                        self.position = next_room_id;
                    }
                    // Sinon, message déjà affiché par tenter_ouverture
                } else {
                    // Sinon, met à jour la position du personnage vers la nouvelle salle
                    self.position = next_room_id;
                }
            } else {
                println!("❌ Salle inconnue.");
            }
        } else {
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

    pub fn defense(&self) -> i32 {
        self.vivant.defense()
    }

    pub fn is_alive(&self) -> bool{

        self.vivant.health() > 0
    }

    pub fn get_item_details<'a>(&self, item_id: u32, items: &'a [Item]) -> Option<&'a Item> {
        items.iter().find(|i| i.id() == item_id)
    }

    pub fn get_active_quests(&self, all_quests: &HashMap<u32, Quete>, items: &Vec<Item>, enemies: &HashMap<u32, Enemy>) -> Vec<String> {
        // Create a vector to store the names of matching quests.
        let mut quest_titles: Vec<String> = vec![];

        // Iterate over each quest ID in the character's quests list.
        for &quest_id_from_char in &self.quests {
            // Find the quest in the all_quests list that matches the ID.
            // Clone the name and push it to the quest_titles vector.
            let mut descriptor = String::from("* ");

            if let Some(quest_found) = all_quests.get(&quest_id_from_char) {
                // Clone the name and append it to descriptor
                descriptor.push_str(&format!("{} - {}: ", quest_found.name(), quest_found.objectif_type));

                if quest_found.objectif_type == "collecter" {
                    let Some(item) = items.iter().find(|i| i.id() == quest_found.objectif.collecter.item_id) else { todo!() };

                    descriptor.push_str(&format!("{} - {}",
                                                 quest_found.objectif.collecter.target,
                                                 item.name()
                    ));
                } else if quest_found.objectif_type == "tuer" {
                    descriptor.push_str(&format!("{} - {}",
                                                 quest_found.objectif.tuer.target,
                                                 enemies.get(&quest_found.objectif.tuer.ennemi_id).unwrap().name()
                    ));
                }

            }

            // Push the constructed string to the vector
            quest_titles.push(descriptor);
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

    pub fn combat<T: Combattant>(&mut self, ennemi: &mut T) -> CombatResult {
        println!("⚔️ Début du combat : {} VS {}", self.name(), ennemi.nom());

        let mut rng = rand::thread_rng();

        loop {
            // ======== Tour du joueur ========
            let esquive = rng.gen_bool(0.1); // 10% de chance d'esquive
            let critique = rng.gen_bool(0.2); // 20% de chance de critique

            let mut degats = self.degats_attaque();
            if critique {
                println!("🎯 Coup critique !");
                degats *= 2;
            }

            println!("👉 {} attaque avec {} dégâts !", self.name(), degats);
            ennemi.infliger_degats(degats);

            if !ennemi.est_vivant() {
                println!("🏆 Tu as vaincu {} !", ennemi.nom());

                let xp_gagnee = 30; // à adapter selon l'ennemi
                self.add_experience(xp_gagnee);
                return CombatResult::VICTORY;
            }

            // ======== Tour de l'ennemi ========
            if esquive {
                println!("🌀 Tu esquives l'attaque de {} !", ennemi.nom());
            } else {
                let degats_ennemi = ennemi.degats_attaque();
                println!("💥 {} attaque avec {} dégâts !", ennemi.nom(), degats_ennemi);
                self.infliger_degats(degats_ennemi);
            }

            println!(
                "❤️ État actuel : {} ({} PV), {} ({} PV)\n",
                self.name(),
                self.sante(),
                ennemi.nom(),
                ennemi.sante()
            );

            if !self.est_vivant() {
                println!("☠️ Tu es mort face à {}…", ennemi.nom());
                return CombatResult::DEFEAT;
            }
        }
    }

    pub fn combat_interactif<T: Combattant>(&mut self, ennemi: &mut T, items: &[Item]) -> CombatResult {
        println!("\n⚔️ Un combat commence contre {} !", ennemi.nom());

        while self.est_vivant() && ennemi.est_vivant() {
            println!(
                "\n🧍‍♂️ {} (PV: {}) vs 🧟 {} (PV: {})",
                self.nom(),
                self.sante(),
                ennemi.nom(),
                ennemi.sante()
            );

            println!("Que veux-tu faire ? (attaquer / utiliser / fuir)");
            print!("> ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "attaquer" => {
                    let critique = rand::thread_rng().gen_bool(0.2);
                    let mut degats = self.degats_attaque().saturating_sub(ennemi.protection_defense());
                    if critique {
                        println!("💥 Coup critique !");
                        degats *= 2;
                    }
                    println!("🗡️ Tu infliges {} dégâts à {}.", degats, ennemi.nom());
                    ennemi.infliger_degats(degats);
                }

                "utiliser" => {
                    self.afficher_inventaire(items);
                    println!("Quel objet veux-tu utiliser ?");
                    print!("> ");
                    stdout().flush().unwrap();
                    let mut nom_objet = String::new();
                    stdin().read_line(&mut nom_objet).unwrap();
                    let nom_objet = nom_objet.trim();
                    self.utiliser_objet(nom_objet, &mut [], items);
                }

                "fuir" => {
                    let chance_fuite = rand::thread_rng().gen_bool(0.5);
                    if chance_fuite {
                        println!("🏃‍♂️ Tu réussis à fuir !");
                        return CombatResult::ONGOING;
                    } else {
                        println!("❌ Échec de la fuite !");
                    }
                }

                _ => {
                    println!("❓ Commande invalide.");
                }
            }

            // === Tour de l'ennemi ===
            if ennemi.est_vivant() {
                let esquive = rand::thread_rng().gen_bool(0.1); // 10% de chance que le joueur esquive
                if esquive {
                    println!("🌀 Tu esquives l'attaque de {} !", ennemi.nom());
                } else {
                    let critique = rand::thread_rng().gen_bool(0.15); // 15% de critique ennemi
                    let mut degats = ennemi.degats_attaque().saturating_sub(self.protection_defense());
                    if critique {
                        println!("💢 Coup critique de {} !", ennemi.nom());
                        degats *= 2;
                    }
                    println!("💥 {} t'attaque et inflige {} dégâts !", ennemi.nom(), degats);
                    self.infliger_degats(degats);
                }
            }
        }

        if self.est_vivant() {
            println!("🎉 Tu as vaincu {} !", ennemi.nom());
            CombatResult::VICTORY
        } else {
            println!("☠️ Tu es mort...");
            CombatResult::DEFEAT
        }
    }



}

impl Tracker for Character {

}

impl Combattant for Character {

    fn nom(&self) -> &str {
        self.name()
    }

    fn force(&self) -> u32 {
        self.strength().max(0) as u32
    }

    fn sante(&self) -> u32 {
        self.health().max(0) as u32
    }

    fn est_vivant(&self) -> bool {
        self.health() > 0
    }

    fn infliger_degats(&mut self, degats: u32) {
        self.set_health( (self.health() - degats as i32).max(0) );
    }

    fn degats_attaque(&self) -> u32 {
        self.strength().max(0) as u32
    }

    fn protection_defense(&self) -> u32 {
        self.defense().max(0) as u32
    }



}