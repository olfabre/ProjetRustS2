use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde::de::Visitor;

use crate::models::traits::descriptible::Descriptible;
use crate::models::{entities::room::Room, entities::item::Item};
use crate::models::dialogue::Dialogue;
use crate::models::entities::Enemy::Enemy;
use crate::models::entities::inventory::Inventory;
use crate::models::entities::quete::Quete;
use crate::models::entities::vivant::Vivant;
use crate::models::tracker::Tracker;
use crate::models::traits::combattant::{CombatResult, Combattant};
use crate::models::traits::money_manager::MoneyManager;

// Structure qui représente le personnage joueur
// Hérite des attributs de base d'une entité vivante et ajoute des fonctionnalités spécifiques au joueur
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
<<<<<<< HEAD
    pub(crate) vivant: Vivant,    // Attributs de base d'une entité vivante (stats, inventaire, etc.)
    pub position: usize,          // ID de la salle où se trouve le personnage
    pub level: i32,               // Niveau du joueur
    pub experience: i32,          // Points d'expérience accumulés
    pub money: i32,               // Argent du joueur
    pub quests: Vec<u32>          // Liste des IDs des quêtes actives
}

// Implémentation du trait Descriptible pour afficher les informations du personnage
=======
    // Structure principale du personnage combinant les attributs liés au jeu
    pub(crate) vivant: Vivant,      // Structure Vivant intégrée, probablement contient des statistiques comme la santé et l'inventaire
    pub position: usize,            // ID de la salle actuelle (index dans une liste de salles)
    pub level: i32,                 // Niveau actuel du joueur
    pub experience: i32,            // Points d'expérience accumulés
    pub money: i32,                 // Argent ou monnaie du joueur
    pub quests: Vec<u32>            // Liste des IDs de quêtes actives
}



>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
impl Descriptible for Character {
    // Renvoie une chaîne de description du personnage comprenant le nom et les statistiques principales
    fn get_description(&self) -> String {
        format!(
            "{} (Santé: {}, Force: {}, Intelligence: {})",
            self.name(), self.health(), self.strength(), self.intelligence()
        )
    }
}

impl Character {
    // Déplace le personnage dans une nouvelle salle si possible
    // Vérifie si la direction existe et si la salle n'est pas verrouillée
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

<<<<<<< HEAD
    // Permet au personnage de ramasser un objet dans la salle actuelle
    // Met à jour l'inventaire et les quêtes associées
=======
    /// Permet au personnage de récupérer un objet dans la pièce actuelle par son nom.
    /// Ajoute l'objet à l'inventaire s'il y a de la place et le retire de la pièce.
    /// Déclenche la logique de suivi de quête si nécessaire.
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
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

<<<<<<< HEAD
    // Permet au personnage d'utiliser un objet de son inventaire
    // Applique les effets spécifiques selon le type d'objet
=======
    /// Utilise un objet de l'inventaire du personnage par son nom.
    /// Applique l'effet de l'objet (soins, déverrouillage, amélioration des statistiques, etc.).
    /// L'objet est retiré de l'inventaire s'il s'agit d'un consommable.
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
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

<<<<<<< HEAD
    // Augmente le niveau du personnage et ses statistiques
=======
    /// Augmente le niveau du personnage.
    /// Améliore les statistiques de base telles que la santé, la force et l'intelligence.
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
    pub fn level_up(&mut self) {
        self.level += 1;
        self.set_health(self.health() + 20);
        self.set_strength(self.strength() + 2);
        self.set_intelligence(self.intelligence() + 2);
        println!("🔥 Vous montez au niveau {} ! Vos statistiques augmentent.", self.level);
    }

<<<<<<< HEAD
    // Ajoute de l'expérience au personnage et vérifie si un niveau supérieur est atteint
=======
    /// Ajouter experience
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
    pub fn add_experience(&mut self, xp: i32) {
        println!("🎖️ Vous gagnez {} XP !", xp);
        self.experience += xp;

        // Vérifier si le joueur atteint le niveau suivant
        while self.experience >= self.level * 100 {
            self.level_up();
        }
    }

<<<<<<< HEAD
    // Affiche le contenu de l'inventaire du personnage
=======
    /// Affiche le contenu actuel de l'inventaire du personnage.
    /// Répertorie les noms des objets, leurs quantités et leurs effets.
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
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

    // Supprime une quête de la liste des quêtes actives
    pub fn supprimer_quete(&mut self, id: u32) {
        self.quests.retain(|&q| q != id);
    }

    // Getters pour les attributs de base
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

    // Setters pour modifier les statistiques
    pub fn set_health(&mut self, health: i32) {
        self.vivant.set_health(health);
    }

    pub fn set_strength(&mut self, strength: i32) {
        self.vivant.set_strength(strength);
    }

    pub fn set_intelligence(&mut self, intelligence: i32) {
        self.vivant.set_intelligence(intelligence);
    }

    // Getter mutable pour l'inventaire
    pub fn inventory_mut(&mut self) -> &mut Inventory {
        self.vivant.inventory_mut()
    }

    // Getter pour la défense
    pub fn defense(&self) -> i32 {
        self.vivant.defense()
    }

    // Vérifie si le personnage est en vie
    pub fn is_alive(&self) -> bool {
        self.vivant.health() > 0
    }

<<<<<<< HEAD
    // Récupère les détails d'un objet spécifique dans l'inventaire
    pub fn get_item_details<'a>(&self, item_id: u32, items: &'a [Item]) -> Option<&'a Item> {
        items.iter().find(|i| i.id() == item_id)
    }

    // Récupère la liste des quêtes actives avec leurs détails
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

    // Getter pour la liste des quêtes
=======
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
    pub fn quests(&self) -> &Vec<u32> {
        &self.quests
    }

<<<<<<< HEAD
    // Ajoute de l'argent au personnage
    pub fn add_argent(&mut self, quantité: i32) {
        self.money += quantité;
    }

    // Gère un combat automatique avec un ennemi
=======

    /// Renvoie une liste des descriptions de quêtes actuellement actives pour le personnage.
    /// Associe les identifiants de quête aux données de quête et formate l'objectif pour l'affichage.
    pub fn get_active_quests(&self, all_quests: &HashMap<u32, Quete>, items: &Vec<Item>, enemies: &HashMap<u32, Enemy>) -> Vec<String> {
        // Créez un vecteur pour stocker les descriptions des quêtes correspondantes.
        let mut quest_titles: Vec<String> = vec![];

        // Itérer sur chaque ID de quête dans la liste des quêtes du personnage.
        for &quest_id_from_char in &self.quests {
            // Initialise une chaîne de caractères pour stocker la description formatée de la quête.
            let mut descriptor = String::from("* ");

            // Vérifie si la quête correspondant à l'ID existe dans la liste all_quests.
            if let Some(quest_found) = all_quests.get(&quest_id_from_char) {
                // Ajoute le nom et le type de l'objectif de la quête au descripteur.
                descriptor.push_str(&format!("{} - {}: ", quest_found.name(), quest_found.objectif_type));

                // Vérifie le type d'objectif de la quête.
                if quest_found.objectif_type == "collecter" {
                    // Recherche l'objet correspondant à l'ID de l'objectif de collecte.
                    let Some(item) = items.iter().find(|i| i.id() == quest_found.objectif.collecter.item_id) else { todo!() };

                    // Ajoute le nombre d'objets à collecter et leur nom au descripteur.
                    descriptor.push_str(&format!("{} - {}", quest_found.objectif.collecter.target, item.name()));
                } else if quest_found.objectif_type == "tuer" {
                    // Ajoute le nombre d'ennemis à éliminer et leur nom au descripteur.
                    descriptor.push_str(&format!("{} - {}", quest_found.objectif.tuer.target, enemies.get(&quest_found.objectif.tuer.ennemi_id).unwrap().name()));
                }
            }

            // Ajoute la description formatée de la quête au vecteur quest_titles.
            quest_titles.push(descriptor);
        }

        // Renvoie la liste des descriptions de quêtes formatées.
        quest_titles
    }

    /// Gère une boucle de combat complète entre le joueur et un seul ennemi.
    /// Alterne les tours, calcule les dégâts, applique les coups critiques et l'esquive.
    /// Accorde de l'expérience en cas de victoire et gère les conditions de défaite.
>>>>>>> 45c85e3a97d5a1eb3c0ba5f1740b97e47a47678f
    pub fn combat<T: Combattant>(&mut self, ennemi: &mut T) -> CombatResult {
        println!("⚔️ Début du combat : {} VS {}", self.name(), ennemi.nom());

        let mut rng = rand::thread_rng();

        loop {
            // ======== Tour du joueur ========
            let esquive = rng.gen_bool(0.1); // 10% de chance d'esquive
            let critique = rng.gen_bool(0.2); // 20% de chance de critique

            let mut degats = self.degats_attaque();
            if critique {
                println!("�� Coup critique !");
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

    /// Boucle de combat interactive où l'utilisateur choisit des actions (attaquer, utiliser un objet, fuir).
    /// Fournit une interaction dynamique au tour par tour avec une entrée en ligne de commande.
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

impl MoneyManager for Character {

    fn money_mut(&mut self) -> &mut i32 {
        &mut self.money
    }
}