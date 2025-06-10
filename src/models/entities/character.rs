use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde::de::Visitor;

use crate::models::traits::descriptible::Descriptible;
use crate::models::{entities::room::Room, entities::item::Item};
use crate::models::dialogue::Dialogue;
use crate::models::entities::inventory::Inventory;
use crate::models::entities::inventory_item::InventoryItem;
use crate::models::entities::loot_entry::LootEntry;
use crate::models::entities::quete::Quete;
use crate::models::entities::vivant::Vivant;
use crate::models::game::Game;
use crate::models::traits::combattant::{CombatResult, Combattant};
use crate::models::traits::quete_manager::QueteManager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    // Structure principale du personnage combinant les attributs liés au jeu
    pub(crate) vivant: Vivant,      // Structure Vivant intégrée, probablement contient des statistiques comme la santé et l'inventaire
    pub position: usize,            // ID de la salle actuelle (index dans une liste de salles)
    pub level: i32,                 // Niveau actuel du joueur
    pub experience: i32,            // Points d'expérience accumulés
    pub money: i32,                 // Argent ou monnaie du joueur
    pub quests: Vec<u32>            // Liste des IDs de quêtes actives
}



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
                    println!("🏃 {} se déplace vers la salle : {}", self.name(), next_room.name());
                }
            } else {
                println!("❌ Salle inconnue.");
            }
        } else {
            println!("❌ Direction invalide.");
        }
    }

    /// Permet au personnage de récupérer un objet dans la pièce actuelle par son nom.
    /// Ajoute l'objet à l'inventaire s'il y a de la place et le retire de la pièce.
    /// Déclenche la logique de suivi de quête si nécessaire.
    pub fn prendre_objet(&mut self, objet_nom: &str, game: &mut Game) {
        // On convertit le nom pour ignorer la casse lors de la comparaison
        let objet_nom = objet_nom.to_lowercase();
        let current_room = &mut game.rooms[self.position];

        // On cherche l'objet dans la salle actuelle par nom (case-insensitive)
        if let Some((index, item_id)) = current_room
            .items
            .iter()
            .enumerate()
            .find_map(|(i, id)| {
                game.items
                    .iter()
                    .find(|item| item.id() == *id && item.name().to_lowercase() == objet_nom)
                    .map(|_| (i, *id))
            })
        {
            if let Some(item) = game.items.iter().find(|item| item.id() == item_id) {
                let ajouté = self.vivant.inventory.add_item(item.id(), 1);

                if ajouté {
                    current_room.items.remove(index); // Retirer l'objet de la salle
                    println!("👜 Tu as ramassé '{}'.", item.name());
                    self.track_item(item_id, &mut game.quetes, &mut game.dialogues);
                } else {
                    println!("❌ Inventaire plein, impossible de ramasser '{}'.", item.name());
                }
            }
        } else {
            println!("❌ Aucun objet nommé '{}' trouvé ici.", objet_nom);
        }
    }

    /// Utilise un objet de l'inventaire du personnage par son nom.
    /// Applique l'effet de l'objet (soins, déverrouillage, amélioration des statistiques, etc.).
    /// L'objet est retiré de l'inventaire s'il s'agit d'un consommable.
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
        // Incrémente le niveau du personnage de 1
        self.level += 1;
        // Augmente la santé de 20 points
        self.set_health(self.health() + 20);
        // Augmente la force de 2 points
        self.set_strength(self.strength() + 2);
        // Augmente l’intelligence de 2 points
        self.set_intelligence(self.intelligence() + 2);
        // Affiche un message de montée de niveau
        println!("🔥 Vous montez au niveau {} ! Vos statistiques augmentent.", self.level);
    }

    // Ajoute de l'expérience au personnage
    // Vérifie si un passage de niveau est possible
    pub fn add_experience(&mut self, xp: i32) {
        // Ajoute l'expérience au total du personnage
        self.experience += xp;

        // Initialise un compteur de montées de niveau
        let mut montées = 0;

        // Boucle tant que le personnage a assez d'XP pour monter de niveau
        while self.experience >= self.level * 100 {
            // Incrémente le niveau
            self.level_up();
            // Compte le nombre de niveaux gagnés
            montées += 1;
        }

        // Affiche le nombre de niveaux gagnés
        if montées > 0 {
            println!("🆙 Tu as gagné {} niveau{} !", montées, if montées > 1 { "x" } else { "" });
        }
        // Affiche le niveau actuel
        println!("📈 Niveau actuel : {}", self.level);
    }


    /// Affiche le contenu actuel de l'inventaire du personnage.
    /// Répertorie les noms des objets, leurs quantités et leurs effets.
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

    pub fn quests(&self) -> &Vec<u32> {
        &self.quests
    }



    // Affiche les objets récupérés après un combat et les ajoute à l'inventaire du joueur.
    //loot – Liste des objets à ajouter, avec leur identifiant et quantité
    // items – Liste de tous les objets possibles, utilisée pour retrouver les noms
    pub fn afficher_et_ajouter_loot(&mut self, loot: &[InventoryItem], items: &[Item]) {
        // Vérifie si aucun objet n’a été trouvé
        if loot.is_empty() {
            println!("🎁 Aucun objet trouvé.");
        } else {
            println!("🎁 Loot récupéré :");

            // Parcours chaque objet récupéré
            for obj in loot {
                // Recherche le nom de l’objet à partir de son ID
                let nom = items
                    .iter()// Parcours la liste des objets connus
                    .find(|i| i.id() == obj.item_id)// Cherche celui qui a le même ID
                    .map(|i| i.name().as_ref())// Récupère le nom de l’objet
                    .unwrap_or("Objet inconnu");// Si non trouvé, nom par défaut

                // Affiche l’objet et la quantité obtenue
                println!("- {} x{}", nom, obj.quantity);

                // Ajoute l’objet dans l’inventaire du joueur
                self.inventory_mut().add_item(obj.item_id, obj.quantity);
            }
        }
    }



    /// Lance un combat interactif entre le joueur et un ennemi donné.
    ///
    /// Le joueur peut choisir d'attaquer, d'utiliser un objet ou de fuir.
    /// Si le joueur gagne, il obtient du loot et de l'expérience.
    ///
    /// # Paramètres
    /// - `ennemi` : l'ennemi à combattre.
    /// - `items` : la liste globale des objets connus du jeu.
    ///
    /// # Retour
    /// Un `CombatResult` indiquant si le joueur a gagné, perdu, ou fui.
    pub fn combat_interactif<T: Combattant>(&mut self, ennemi: &mut T, items: &[Item]) -> CombatResult {
        println!("\n⚔️ Un combat commence contre {} !", ennemi.nom());

        // Boucle principale : tant que les deux sont vivants, le combat continue
        while self.est_vivant() && ennemi.est_vivant() {
            // Affiche les PV actuels des deux combattants
            println!(
                "\n🧍‍♂️ {} (PV: {}) vs 🧟 {} (PV: {})",
                self.nom(),
                self.sante(),
                ennemi.nom(),
                ennemi.sante()
            );

            // Demande à l'utilisateur l'action à effectuer
            println!("Que veux-tu faire ? (attaquer / utiliser / fuir)");
            print!("> ");
            stdout().flush().unwrap(); // Assure l'affichage immédiat du prompt
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap(); // Lecture de l'entrée utilisateur
            let input = input.trim().to_lowercase(); // Nettoie et convertit en minuscule

            // Traitement du choix de l'utilisateur
            match input.as_str() {
                "attaquer" => {
                    // Génère une chance de critique (20%)
                    let critique = rand::thread_rng().gen_bool(0.2);
                    // Calcule les dégâts nets après défense
                    let mut degats = self.degats_attaque().saturating_sub(ennemi.protection_defense());
                    if critique {
                        // Double les dégâts si critique
                        println!("💥 Coup critique !");
                        degats *= 2;
                    }

                    // Affiche les dégâts infligés
                    println!("🗡️ Tu infliges {} dégâts à {}.", degats, ennemi.nom());
                    ennemi.infliger_degats(degats);
                }

                "utiliser" => {
                    // Affiche l'inventaire disponible
                    self.afficher_inventaire(items);
                    println!("Quel objet veux-tu utiliser ?");
                    print!("> ");
                    stdout().flush().unwrap();
                    let mut nom_objet = String::new();
                    stdin().read_line(&mut nom_objet).unwrap();
                    let nom_objet = nom_objet.trim();
                    // Utilise l’objet choisi
                    self.utiliser_objet(nom_objet, &mut [], items);
                }

                "fuir" => {
                    // Chance de fuir avec succès (50%)
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

            // === Tour de l'ennemi s’il est encore vivant ===
            if ennemi.est_vivant() {
                // Chance d’esquive (10%)
                let esquive = rand::thread_rng().gen_bool(0.1); // 10% de chance que le joueur esquive
                if esquive {
                    println!("🌀 Tu esquives l'attaque de {} !", ennemi.nom());
                } else {
                    // Chance de coup critique ennemi (15%)
                    let critique = rand::thread_rng().gen_bool(0.15); // 15% de critique ennemi
                    let mut degats = ennemi.degats_attaque().saturating_sub(self.protection_defense());
                    if critique {
                        println!("💢 Coup critique de {} !", ennemi.nom());
                        degats *= 2;
                    }
                    // Applique les dégâts au joueur
                    println!("💥 {} t'attaque et inflige {} dégâts !", ennemi.nom(), degats);
                    self.infliger_degats(degats);
                }
            }
        }

        if self.est_vivant() {
            // Le joueur a gagné
            println!("\n🏆 Combat terminé !");
            println!("🎉 Tu as vaincu {} !", ennemi.nom());

            // Génération du butin (loot) à partir de la table de l’ennemi
            let loot = LootEntry::generer_depuis_table(ennemi.loot());
            // Affichage et ajout dans l’inventaire
            self.afficher_et_ajouter_loot(&loot, items);

            // Ajout de chaque item au vrai inventaire du joueur
            for item in loot {
                self.vivant.inventory_mut().add_item(item.item_id, item.quantity);
            }


            // Gain d’expérience après victoire
            let xp = ennemi.experience_gain();
            self.add_experience(xp);
            println!("🎖️ Expérience gagnée : {} XP !", xp);
            println!(" ❤️ Santé restante : {} ", self.sante());


            CombatResult::VICTORY
        } else {
            // Le joueur a perdu
            println!("☠️ Tu es mort...");
            CombatResult::DEFEAT
        }
    }

}



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

