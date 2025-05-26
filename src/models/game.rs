use crate::models::{entities::character::Character, entities::room::Room, entities::item::Item, entities::pnj::Pnj, dialogue::Dialogue, entities::ennemie::Enemy};
use crate::io::loader::{load_characters_from_file, load_dialogues_from_file, load_items_from_file, load_pnjs_from_file, load_room_from_file, load_ennemie_from_file};
use std::io::stdin;
use crate::traits::combattant::{CombatResult, Combattant};


pub struct Game {
    rooms: Vec<Room>,
    characters: Vec<Character>,
    items: Vec<Item>,
    pnjs: Vec<Pnj>,
    dialogues: Vec<Dialogue>,
    ennemies: Vec<Enemy>,
}

impl Game {
    fn get_item_name(&self, item_id: u32) -> String {
        self.items
            .iter()
            .find(|item| item.id() == item_id)
            .map(|item| item.name().to_string())
            .unwrap_or_else(|| format!("Objet inconnu (ID: {})", item_id))
    }

    /// Crée une nouvelle instance du jeu en chargeant les données depuis les fichiers JSON
    pub fn new() -> Self {
        let rooms = load_room_from_file("data/rooms.json").expect("Erreur lors du chargement des salles.");
        let characters = load_characters_from_file("data/characters.json").expect("Erreur lors du chargement des personnages.");
        let items = load_items_from_file("data/items.json").expect("Erreur lors du chargement des objets.");
        let pnjs = load_pnjs_from_file("data/pnjs.json").expect("Erreur lors du chargement des PNJ.");
        let dialogues = load_dialogues_from_file("data/dialogue.json").expect("Erreur lors du chargement des PNJ");
        let ennemies = load_ennemie_from_file("data/ennemie.json").expect("Erreur lors du chargement des ennemis.");

        Game { rooms, characters, items, pnjs, dialogues, ennemies }
    }

    /// Démarre la boucle principale du jeu
    pub fn run(&mut self) {
        let items_ref = &self.items;
        if let Some(character) = self.characters.first_mut() {
            loop {
                let current_room = &self.rooms[character.position];

                println!("\n🌍 {} est actuellement dans : {}", character.name(), current_room.name());
                println!("📍 {} : {}", current_room.elem.name(), current_room.elem.description());

                // Affichage des objets trouvés dans la salle
                if !current_room.items.is_empty() {
                    println!("🛠 Objets trouvés :");
                    for &item_id in &current_room.items {
                        if let Some(item) = self.items.iter().find(|i| i.id() == item_id) {
                            println!("- {} : {} (Effet : {})", item.name(), item.description(), item.effect());
                        }
                    }
                } else {
                    println!("🛠 Aucun objet trouvable ici.");
                }

                // Vérifie si la salle contient des ennemis
                if !current_room.enemies.is_empty() {
                    println!("⚔️ Ennemis présents ici :");
                    for &ennemie_id in &current_room.enemies {
                        // Recherche de l’ennemi correspondant dans la liste globale
                        if let Some(ennemie) = self.ennemies.iter().find(|e| e.id() == ennemie_id) {
                            println!(
                                "   - {} (PV: {}, Force: {}, Agilité: {})",
                                ennemie.vivant.name(), ennemie.vivant.health(), ennemie.vivant.strength(), ennemie.agility
                            );
                        }
                    }
                } else {
                    // Aucun ennemi trouvé dans cette salle
                    println!("⚔️ Aucun ennemi présent ici.");
                }


                // Affichage des PNJ présents dans la salle
                if !current_room.pnjs.is_empty() {
                    println!("🧑‍🤝‍🧑 Personnages présents :");
                    for &pnj_id in &current_room.pnjs {
                        if let Some(pnj) = self.pnjs.iter().find(|p| p.id() == pnj_id) {
                            println!("- {}", pnj.name());
                        }
                    }
                } else {
                    println!("🧑‍🤝‍🧑 Aucun personnage ici.");
                }

                println!("\nOù veux-tu aller ? (north, south, east, west, up, down, tunnel, quit, prendre <objet>, utiliser <objet>, parler <pnj>, combattre <ennemie>)");

                // Affiche les directions disponibles
                println!("Sorties disponibles :");
                for direction in current_room.exits.keys() {
                    println!("- {}", direction);
                }



                // Lecture de l'entrée utilisateur
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Erreur de lecture (UTF-8 invalide) : {}", e);
                        continue;
                    }
                }
                let input = input.trim().to_lowercase();

                if input == "quit" {
                    println!("🏁 Fin du jeu.");
                    break;
                }

                // Prendre un objet
                if input.starts_with("prendre ") {
                    let objet_nom = &input[8..].trim().to_lowercase();
                    character.prendre_objet(objet_nom, &mut self.rooms, &self.items);
                    continue;
                }

                //Utiliser les objtes
                if input.starts_with("utiliser ") {
                    let objet_nom = &input[8..].trim();
                    character.utiliser_objet(objet_nom, &mut self.rooms, &self.items);
                    continue;
                }


                // Parler à un PNJ
                if input.starts_with("parler ") {
                    let pnj_nom = &input[7..].trim();
                    Pnj::parler_au_pnj(pnj_nom, character, &self.rooms, &self.pnjs, &self.dialogues);
                    continue;
                }

                // Combattre un ennemi
                // Combat
                if input.starts_with("combattre ") {
                    let ennemi_nom = &input[10..].trim().to_lowercase();
                    let current_room_id = character.position as u32;

                    if let Some(enemy_index) = self.ennemies.iter().position(|e| {
                        e.room_id == current_room_id && e.vivant.name().to_lowercase() == *ennemi_nom
                    }) {
                        let mut enemy = self.ennemies[enemy_index].clone();

                        // 🔄 Utilisation du résultat du combat
                        match character.combat_interactif(&mut enemy, &self.items) {
                            CombatResult::VICTORY => {
                                let loot = enemy.drop_loot();
                                let mut loot_affichage = vec![];

                                for inv_item in &loot {
                                    let name = self.items
                                        .iter()
                                        .find(|i| i.id() == inv_item.item_id)
                                        .map(|i| i.name().to_string())
                                        .unwrap_or_else(|| format!("Objet inconnu ({})", inv_item.item_id));
                                    loot_affichage.push(format!("{} x{}", name, inv_item.quantity));
                                }

                                println!("\n🎉 Victoire contre {} !", enemy.vivant.name());

                                if loot_affichage.is_empty() {
                                    println!("🎁 Aucun objet trouvé.");
                                } else {
                                    println!("🎁 Loot récupéré :");
                                    for ligne in loot_affichage {
                                        println!("- {}", ligne);
                                    }
                                }

                                println!("🩸 Santé restante : {} PV", character.vivant.health());

                                for item in loot {
                                    character.vivant.inventory.add_item(item.item_id, item.quantity);
                                }

                                if let Some(room) = self.rooms.get_mut(character.position) {
                                    room.enemies.retain(|&id| id != enemy.vivant.id());
                                }
                                self.ennemies.remove(enemy_index);
                            }

                            CombatResult::DEFEAT => {
                                println!("☠️ Tu es mort… fin de l'aventure.");
                                break;
                            }

                            CombatResult::ONGOING => {
                                println!("🔙 Tu as fui le combat.");
                            }
                            _ => {}
                        }
                    } else {
                        println!("❌ Aucun ennemi nommé '{}' ici.", ennemi_nom);
                    }

                    continue;
                }




                // Traduire les directions anglaises vers les directions du fichier JSON
                let direction = match input.as_str() {
                    "north" => "Nord",
                    "south" => "Sud",
                    "east" => "Est",
                    "west" => "Ouest",
                    "up" => "À l'étage",
                    "down" => "Sous-sol",
                    "tunnel" => "Tunnel",
                    "rez-de-chaussée" => "Rez-de-chaussée",
                    _ => input.as_str(),
                };

                // Déplacement du personnage avec vérification
                character.try_move(direction, &self.rooms);
            }
        }
    }
}
