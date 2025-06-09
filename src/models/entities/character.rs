// Module de gestion du personnage joueur
// Définit la structure Character et ses fonctionnalités
// Gère toutes les actions possibles du joueur dans le jeu

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
use crate::models::entities::inventory_item::InventoryItem;
use crate::models::entities::loot_entry::LootEntry;
use crate::models::entities::quete::Quete;
use crate::models::entities::vivant::Vivant;
use crate::models::tracker::Tracker;
use crate::models::traits::combattant::{CombatResult, Combattant};
use crate::models::traits::money_manager::MoneyManager;

// Structure principale du personnage joueur
// Hérite des propriétés de base d'un Vivant et ajoute des fonctionnalités spécifiques
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub(crate) vivant: Vivant,        // Propriétés de base (santé, inventaire, etc.)
    pub position: usize,              // ID de la salle actuelle
    pub level: i32,                   // Niveau du personnage
    //pub experience: i32,              // Points d'expérience
    pub money: i32,                   // Argent possédé
    pub quests: Vec<u32>              // Liste des quêtes actives
}

// Implémentation du trait Descriptible pour Character
// Permet d'obtenir une description complète du personnage
impl Descriptible for Character {
    fn get_description(&self) -> String {
        format!(
            "{} (Santé: {}, Force: {}, Intelligence: {})",
            self.name(), self.health(), self.strength(), self.intelligence()
        )
    }
}

impl Character {
    // Tente de déplacer le personnage dans une direction
    // Vérifie si la direction est valide et si la salle n'est pas verrouillée
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
                    println!("🏃 {} se déplace vers la salle : {}", self.name(), next_room.name());
                }
            } else {
                println!("❌ Salle inconnue.");
            }
        } else {
            println!("❌ Direction invalide.");
        }
    }

    // Permet au personnage de ramasser un objet dans la salle actuelle
    // Met à jour l'inventaire et les quêtes associées
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

    // Permet au personnage d'utiliser un objet de son inventaire
    // Applique les effets de l'objet selon son type
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

    // Fait monter le personnage d'un niveau
    // Augmente ses statistiques de base
    pub fn level_up(&mut self) {
        self.level += 1;
        self.set_health(self.health() + 20);
        self.set_strength(self.strength() + 2);
        self.set_intelligence(self.intelligence() + 2);
        println!("🔥 Vous montez au niveau {} ! Vos statistiques augmentent.", self.level);
    }

    // Ajoute de l'expérience au personnage
    // Vérifie si un passage de niveau est possible
    pub fn add_experience(&mut self, xp: i32) {
        println!("🎖️ Vous gagnez {} XP !", xp);
        self.vivant.experience += xp;
        let mut montées = 0;

        // Vérifier si le joueur atteint le niveau suivant
        while self.vivant.experience >= self.level * 100 {
            self.level_up();
            montées += 1;
        }
        if montées > 0 {
            println!("🆙 Tu as gagné {} niveau{} !", montées, if montées > 1 { "x" } else { "" });
        }
        println!("📈 Niveau actuel : {}", self.level);
    }

    // Affiche le contenu de l'inventaire du personnage
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

    // Ajoute une quête à la liste des quêtes actives
    pub fn ajouter_quete(&mut self, id: u32) {
        self.quests.push(id);
    }

    // Retire une quête de la liste des quêtes actives
    pub fn supprimer_quete(&mut self, id: u32) {
        self.quests.retain(|&q| q != id);
    }

    // Retourne l'identifiant unique du personnage
    pub fn id(&self) -> u32 {
        self.vivant.id()
    }

    // Retourne le nom du personnage
    pub fn name(&self) -> &str {
        self.vivant.name()
    }

    // Retourne les points de vie actuels
    pub fn health(&self) -> i32 {
        self.vivant.health()
    }

    // Retourne la force du personnage
    pub fn strength(&self) -> i32 {
        self.vivant.strength()
    }

    // Retourne l'intelligence du personnage
    pub fn intelligence(&self) -> i32 {
        self.vivant.intelligence()
    }

    // Modifie les points de vie du personnage
    pub fn set_health(&mut self, health: i32) {
        self.vivant.set_health(health);
    }

    // Modifie la force du personnage
    pub fn set_strength(&mut self, strength: i32) {
        self.vivant.set_strength(strength);
    }

    // Modifie l'intelligence du personnage
    pub fn set_intelligence(&mut self, intelligence: i32) {
        self.vivant.set_intelligence(intelligence);
    }

    // Retourne une référence mutable à l'inventaire
    pub fn inventory_mut(&mut self) -> &mut Inventory {
        self.vivant.inventory_mut()
    }

    // Retourne la défense du personnage
    pub fn defense(&self) -> i32 {
        self.vivant.defense()
    }

    // Vérifie si le personnage est en vie
    pub fn is_alive(&self) -> bool {
        self.vivant.health() > 0
    }

    // Retourne les détails d'un objet de l'inventaire
    pub fn get_item_details<'a>(&self, item_id: u32, items: &'a [Item]) -> Option<&'a Item> {
        items.iter().find(|i| i.id() == item_id)
    }

    // Retourne la liste des quêtes actives avec leur progression
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

    // Retourne la liste des quêtes actives
    pub fn quests(&self) -> &Vec<u32> {
        &self.quests
    }

    // Ajoute de l'argent au personnage
    pub fn add_argent(&mut self, quantité: i32) {
        self.money += quantité;
    }

    pub fn afficher_et_ajouter_loot(&mut self, loot: &[InventoryItem], items: &[Item]) {
        if loot.is_empty() {
            println!("🎁 Aucun objet trouvé.");
        } else {
            println!("🎁 Loot récupéré :");
            for obj in loot {
                let nom = items
                    .iter()
                    .find(|i| i.id() == obj.item_id)
                    .map(|i| i.name().as_ref())
                    .unwrap_or("Objet inconnu");
                println!("- {} x{}", nom, obj.quantity);
                self.inventory_mut().add_item(obj.item_id, obj.quantity);
            }
        }
    }



    // Gère un combat interactif avec un ennemi
    // Permet au joueur de choisir ses actions
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
            println!("\n🏆 Combat terminé !");
            println!("🎉 Tu as vaincu {} !", ennemi.nom());

            let loot = LootEntry::generer_depuis_table(ennemi.loot());
            self.afficher_et_ajouter_loot(&loot, items);

            // Ajout dans l’inventaire
            for item in loot {
                self.vivant.inventory_mut().add_item(item.item_id, item.quantity);
            }


            let xp = ennemi.experience_gain();
            self.add_experience(xp);
            println!("🎖️ Expérience gagnée : {} XP !", xp);
            println!(" ❤️ Santé restante : {} ", self.sante());


            CombatResult::VICTORY
        } else {
            println!("☠️ Tu es mort...");
            CombatResult::DEFEAT
        }
    }

}

// Implémentation du trait Tracker pour Character
// Permet de suivre la progression des quêtes
impl Tracker for Character {}

// Implémentation du trait Combattant pour Character
// Permet au personnage de participer aux combats
impl Combattant for Character {
    // Retourne le nom du personnage
    fn nom(&self) -> &str {
        self.name()
    }

    // Retourne la force du personnage (minimum 0)
    fn force(&self) -> u32 {
        self.strength().max(0) as u32
    }

    // Retourne les points de vie du personnage (minimum 0)
    fn sante(&self) -> u32 {
        self.health().max(0) as u32
    }

    // Vérifie si le personnage est en vie
    fn est_vivant(&self) -> bool {
        self.health() > 0
    }

    // Inflige des dégâts au personnage
    fn infliger_degats(&mut self, degats: u32) {
        self.set_health( (self.health() - degats as i32).max(0) );
    }

    // Retourne les dégâts d'attaque du personnage
    fn degats_attaque(&self) -> u32 {
        self.strength().max(0) as u32
    }

    // Retourne la protection défensive du personnage
    fn protection_defense(&self) -> u32 {
        self.defense().max(0) as u32
    }

    fn loot(&self) -> &[LootEntry] {
        &[]
    }

    fn experience_gain(&self) -> i32 {
        self.vivant.strength() * 2
    }
}

// Implémentation du trait MoneyManager pour Character
// Permet de gérer l'argent du personnage
impl MoneyManager for Character {
    fn money_mut(&mut self) -> &mut i32 {
        &mut self.money
    }
}