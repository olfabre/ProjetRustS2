use crate::models::{entities::character::Character, entities::room::Room, entities::item::Item, entities::pnj::Pnj, dialogue::Dialogue, entities::Enemy::Enemy};
// use crate::io::loader::{load_characters_from_file, load_dialogues_from_file, load_items_from_file, load_pnjs_from_file, load_room_from_file, load_ennemie_from_file, load_quetes_from_file};
use crate::io::loader::*;
use std::io::{stdin, Write};

use crate::models::entities::quete::Quete;
use std::collections::HashMap;
use log::log;
use crate::models::tracker::Tracker;

pub struct Game {
    rooms: Vec<Room>,
    characters: Vec<Character>,
    items: Vec<Item>,
    pnjs: Vec<Pnj>,
    dialogues: Vec<Dialogue>,
    enemies: HashMap<u32, Enemy>,
    quetes: HashMap<u32, Quete>,
}

impl Game {
    /// Crée une nouvelle instance du jeu en chargeant les données depuis les fichiers JSON
    pub fn new() -> Self {
        // Vectors
        let rooms = load_room_from_file("data/rooms.json").expect("Erreur lors du chargement des salles.");
        let mut characters = load_characters_from_file("data/characters.json").expect("Erreur lors du chargement du joueur.");
        let items = load_items_from_file("data/items.json").expect("Erreur lors du chargement des objets.");
        let pnjs = load_pnjs_from_file("data/pnjs.json").expect("Erreur lors du chargement des PNJ.");
        let mut dialogues = load_dialogues_from_file("data/dialogue.json").expect("Erreur lors du chargement des dialogues");
        // let enemies = load_ennemie_from_file("data/ennemie.json").expect("Erreur lors du chargement des ennemis.");
        // Maps
        let enemies = load_enemies_from_file("data/ennemie.json").expect("Erreur lors du chargement des ennemis.");
        let mut quetes = load_quetes_from_file("data/quetes.json").expect("Erreur lors du chargement des quetes.");

        Game { rooms, characters, items, pnjs, dialogues, enemies, quetes }
    }

    /// Démarre la boucle principale du jeu
    pub fn run(&mut self) {
        if let Some(character) = self.characters.first_mut() {
            loop {
                let current_room = &self.rooms[character.position];
                println!("__________________________________________________________________________________________");
                println!("\n🌍 {} est actuellement dans : {}", character.name(), current_room.name());
                println!("📍 {} : {}", current_room.elem.name(), current_room.elem.description());

                // Affichage des objets trouvés dans la salle
                if !current_room.items.is_empty() {
                    println!("🛠 Objets trouvés :");
                    for &item_id in &current_room.items {
                        if let Some(item) = self.items.iter().find(|i| i.id() == item_id) {
                            println!("   - {} : {} (Effet : {})", item.name(), item.description(), item.effect());
                        }
                    }
                } else {
                    println!("🛠 Aucun objet trouvable ici.");
                }

                // Vérifie si la salle contient des ennemis
                if !current_room.enemies.is_empty() {
                    println!("⚔️ Ennemis présents ici :");
                    for ennemie_id in &current_room.enemies {
                        // Recherche de l’ennemi correspondant dans la liste globale
                        let ennemie = self.enemies.get(ennemie_id);
                        println!(
                            "    - {} (PV: {}, Force: {}, Intelligence: {})",
                            ennemie.unwrap().name(), ennemie.unwrap().health(), ennemie.unwrap().strength(), ennemie.unwrap().intelligence()
                        );

                    }
                } else {
                    // Aucun ennemi trouvé dans cette salle
                    println!("⚔️ Aucun ennemi présent ici.");
                }


                // Affichage des PNJ présents dans la salle
                if !current_room.pnjs.is_empty() {
                    println!("🧑 Personnages présents :");
                    for &pnj_id in &current_room.pnjs {
                        if let Some(pnj) = self.pnjs.iter().find(|p| p.id() == pnj_id) {
                            println!("   - {}", pnj.name());
                        }
                    }
                } else {
                    println!("🧑 Aucun personnage ici.");
                }

                // Affiche les directions disponibles
                println!("🚪 Sorties disponibles :");
                for direction in current_room.exits.keys() {
                    println!("   - {}", direction);
                }

                println!("\nOù veux-tu aller ? ( nord, sud, est, ouest, haut, bas, tunnel, quit )");
                println!("Que veux-tu faire ? ( prendre <objet>, utiliser <objet>, parler <pnj>, combattre <ennemie> )");

                // Lecture de l'entrée utilisateur
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Erreur de lecture");
                let input = input.trim().to_lowercase();

                if input == "quit" {
                    println!("🏁 Fin du jeu.");
                    break;
                }

                // Prendre un objet
                if input.starts_with("prendre ") {
                    let objet_nom = &input[8..].trim().to_lowercase();
                    character.prendre_objet(objet_nom, &mut self.rooms, &self.items, &mut self.quetes, &mut self.dialogues);
                    continue;
                }

                //Utiliser les objets
                if input.starts_with("utiliser ") {
                    let objet_nom = &input[8..].trim();
                    character.utiliser_objet(objet_nom, &mut self.rooms, &self.items);
                    continue;
                }


                // Parler à un PNJ
                if input.starts_with("parler ") {
                    let pnj_nom = &input[7..].trim();
                    Pnj::parler_au_pnj(pnj_nom, character, &self.rooms, &self.pnjs,
                                       &mut self.dialogues, &mut self.quetes, &self.items);
                    continue;
                }

                if input.starts_with("quêtes") {
                    let quetes_found = (character.get_active_quests(&self.quetes));
                    quetes_found.iter().for_each(|quete| println!("{}", quete));
                    continue;
                }

                // Combattre un ennemi
                if input.starts_with("combattre ") {
                    let ennemi_nom = &input[10..].trim().to_lowercase();
                    let current_room_id = character.position.clone();
                    let current_room = self.rooms.iter()
                        .find(|room| room.id() == current_room_id as u32)
                        .expect("La salle actuelle n'a pas été trouvée.");

                    // It might happen that the room contains more than one enemy with the same name,
                    // so we need to check all enemies in the room
                    let enemies: Vec<&Enemy> = current_room.enemies.iter()
                        .filter_map(|enemies_id| self.enemies.get(enemies_id))
                        .filter(|enemy| enemy.name().to_lowercase() == *ennemi_nom)
                        .collect();


                    /*
                        get enemies from the current room
                        if at least one enemy is found, start the combat

                     */

                    if enemies.len() > 0 {

                        // Clone de l'ennemi pour pouvoir le manipuler sans bouger l'original (qui est dans self.ennemies)
                        let enemy_clone = enemies[0].clone();
                        let enemy_id = enemy_clone.id();

                        // Lancement du combat et récupération du résultat (true si ennemi vaincu, false sinon)
                        let ennemi_vaincu = Enemy::fight(character, enemy_clone);

                        if ennemi_vaincu {
                            // Si l’ennemi est vaincu, on le supprime de la salle actuelle
                            if let Some(room) = self.rooms.get_mut(character.position) {
                                room.enemies.retain(|&id| id != enemy_id);
                            }

                            Character::track_enemy(enemy_id, character, &mut self.quetes, &mut self.dialogues);

                            // Suppression de l’ennemi de la liste globale
                            // self.enemies.retain(|e| e.id() != enemy_id);
                        } else if character.health() == 0 {
                            // Si le joueur est mort, on peut afficher un message final et quitter le jeu
                            println!("☠️ Le héros est tombé au combat. Le donjon garde ses secrets... 😔");
                            break; // Sort de la boucle principale -> fin du jeu
                        }
                    } else {
                        println!("❌ Aucun ennemi nommé '{}' ici.", ennemi_nom);
                    }
                }


                // Traduire les directions anglaises vers les directions du fichier JSON
                let direction = match input.as_str() {
                    "nord" => "Nord",
                    "sud" => "Sud",
                    "est" => "Est",
                    "ouest" => "Ouest",
                    "haut" => "À l'étage",
                    "bas" => "Sous-sol",
                    "tunnel" => "Tunnel",
                    "rez-de-chaussée" => "Rez-de-chaussée",
                    _ => input.as_str(),
                };

                // Déplacement du personnage avec vérification
                character.try_move(direction, &self.rooms);
            }
        }
    }

    fn clear_console() {
        print!("\x1B[2J\x1B[1;1H"); // ANSI escape code to clear screen
        std::io::stdout().flush().unwrap(); // Ensure it prints immediately
    }



}
