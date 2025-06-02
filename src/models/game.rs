use rand::Rng;
use std::thread;
use std::time::Duration;

use crate::models::{entities::character::Character, entities::room::Room, entities::item::Item, entities::pnj::Pnj, dialogue::Dialogue, entities::Enemy::Enemy};
// use crate::io::loader::{load_characters_from_file, load_dialogues_from_file, load_items_from_file, load_pnjs_from_file, load_room_from_file, load_ennemie_from_file, load_quetes_from_file};
use crate::io::loader::*;
use std::io::{stdin, Write};

use crate::models::entities::quete::Quete;
use std::collections::HashMap;
use std::io;
use log::log;
use crate::models::tracker::Tracker;
use crate::models::traits::combattant::CombatResult;
use std::process::Command;

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
            // println!("Position de départ du personnage = {}", character.position);
            let mut last_room = character.position;
            
            // Afficher l'image de la salle de départ
            // println!("\nAffichage de l'image de la salle de départ...");
            let current_room = &self.rooms[character.position];
            let room_id = current_room.id();
            
            // Obtenir le chemin absolu du dossier images
            let current_dir = std::env::current_dir().expect("Impossible d'obtenir le répertoire courant");
            let images_dir = current_dir.join("images");
            let image_path = images_dir.join(format!("{}.png", room_id));
            // println!("Chemin complet de l'image recherchée : {}", image_path.display());
            
            if image_path.exists() {
                // println!("Image trouvée : {}", image_path.display());
                let output = Command::new("viu")
                    .arg("-t")  // Afficher dans le terminal
                    .arg(image_path.to_str().unwrap())
                    .spawn();  // Utiliser spawn() au lieu de output()
                
                match output {
                    Ok(_) => (), // println!("Image affichée avec succès"),
                    Err(e) => println!("Erreur lors de l'affichage de l'image : {}\nDétails : {:?}", e, e),
                }
            } else {
                println!("Aucune image trouvée pour la salle {} à {}", room_id, image_path.display());
            }

            loop {
                let current_room = &self.rooms[character.position];
                let room_id = current_room.id();
                // println!("DEBUG - last_room: {}, current position: {}", last_room, character.position);
                
                // Pause execution, waiting for user input
                println!("\n______________________________Appuyez sur Entrée pour continuer___________________________");
                println!("__________________________________________________________________________________________");
                io::stdout().flush().unwrap();  // Ensure the prompt is displayed before waiting
                let _ = io::stdin().read_line(&mut String::new());
                // clear_console();  // Commenté pour tester l'affichage des images

                // Affiche les directions disponibles
                println!("🚪 Sorties disponibles :");
                for direction in current_room.exits.keys() {
                    println!("   - {}", direction);
                }

                // Afficher l'image uniquement si on change de salle
                if last_room != character.position {
                    // println!("\nAffichage de l'image de la salle...");
                    
                    // Obtenir le chemin absolu du dossier images
                    let current_dir = std::env::current_dir().expect("Impossible d'obtenir le répertoire courant");
                    let images_dir = current_dir.join("images");
                    let image_path = images_dir.join(format!("{}.png", room_id));
                    // println!("Chemin complet de l'image recherchée : {}", image_path.display());
                    
                    if image_path.exists() {
                        // println!("Image trouvée : {}", image_path.display());
                        let output = Command::new("viu")
                            .arg("-t")  // Afficher dans le terminal
                            .arg(image_path.to_str().unwrap())
                            .spawn();  // Utiliser spawn() au lieu de output()
                        
                        match output {
                            Ok(_) => (), // println!("Image affichée avec succès"),
                            Err(e) => println!("Erreur lors de l'affichage de l'image : {}\nDétails : {:?}", e, e),
                        }
                    } else {
                        println!("Aucune image trouvée pour la salle {} à {}", room_id, image_path.display());
                    }
                    last_room = character.position;
                }

                println!("\n🌍 {} est actuellement dans : ", character.name());
                println!("   - {} : {}", current_room.elem.name(), current_room.elem.description());

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
                        let ennemie = self.enemies.get(ennemie_id);
                        println!(
                            "    - {} (PV: {}, Force: {}, Intelligence: {})",
                            ennemie.unwrap().name(), ennemie.unwrap().health(), ennemie.unwrap().strength(), ennemie.unwrap().intelligence()
                        );
                    }
                } else {
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

                println!("\nOù veux-tu aller ? ( nord, sud, est, ouest, haut, bas, tunnel, quit )");
                println!("Que veux-tu faire ? ( prendre <objet>, utiliser <objet>, parler <pnj>, combattre <ennemie> )");
                println!("Que veux-tu voir ? ( quêtes, inventaire, stats )");

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
                    let objet_nom = &input[9..].trim();
                    character.utiliser_objet(objet_nom, &mut self.rooms, &self.items);
                    continue;
                }

                // Parler à un PNJ
                if input.starts_with("parler ") {
                    let pnj_nom = &input[7..].trim();
                    Pnj::parler_au_pnj(pnj_nom, character, &self.rooms, &mut self.pnjs,
                                       &mut self.dialogues, &mut self.quetes, &self.items);
                    continue;
                }

                if input.starts_with("quêtes") {
                    let quetes_found = character.get_active_quests(&self.quetes, &self.items, &self.enemies);
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

                    if enemies.len() > 0 {
                        // Clone de l'ennemi pour pouvoir le manipuler sans bouger l'original (qui est dans self.ennemies)
                        let mut enemy_clone = enemies[0].clone();
                        let enemy_id = enemy_clone.id();

                        // 🔄 Utilisation du résultat du combat
                        match character.combat_interactif(&mut enemy_clone, &self.items) {
                            CombatResult::VICTORY => {
                                let loot = enemy_clone.drop_loot();
                                let mut loot_affichage = vec![];

                                for inv_item in &loot {
                                    let name = self.items
                                        .iter()
                                        .find(|i| i.id() == inv_item.item_id)
                                        .map(|i| i.name().to_string())
                                        .unwrap_or_else(|| format!("Objet inconnu ({})", inv_item.item_id));
                                    loot_affichage.push(format!("{} x{}", name, inv_item.quantity));
                                }

                                println!("\n🎉 Victoire contre {} !", enemy_clone.vivant.name());

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
                                    room.enemies.retain(|&id| id != enemy_clone.vivant.id());
                                }

                                Character::track_enemy(enemy_id, character, &mut self.quetes, &mut self.dialogues);
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
                character.try_move(direction, &mut self.rooms);
            }
        }
    }
}

// fn clear_console() {
//     print!("\x1B[2J\x1B[1;1H"); // ANSI escape code to clear screen
//     std::io::stdout().flush().unwrap(); // Ensure it prints immediately
// }