use crate::models::{dialogue::Dialogue, room::Room};
use crate::io::loader::{load_characters_from_file, load_dialogues_from_file, load_items_from_file, load_pnjs_from_file, load_room_from_file};
use std::io::stdin;
use crate::models::elements::character::Character;
use crate::models::elements::item::Item;
use crate::models::elements::pnj::Pnj;

pub struct Game {
    rooms: Vec<Room>,
    characters: Vec<Character>,
    items: Vec<Item>,
    pnjs: Vec<Pnj>,
    dialogues: Vec<Dialogue>,
}

impl Game {
    /// Crée une nouvelle instance du jeu en chargeant les données depuis les fichiers JSON
    pub fn new() -> Self {
        let rooms = load_room_from_file("data/rooms.json").expect("Erreur lors du chargement des salles.");
        let mut characters = load_characters_from_file("data/characters.json").expect("Erreur lors du chargement des personnages.");
        let items = load_items_from_file("data/items.json").expect("Erreur lors du chargement des objets.");
        let pnjs = load_pnjs_from_file("data/pnjs.json").expect("Erreur lors du chargement des PNJ.");
        let dialogues = load_dialogues_from_file("data/dialogue.json").expect("Erreur lors du chargement des PNJ");

        Game { rooms, characters, items, pnjs, dialogues }
    }

    /// Démarre la boucle principale du jeu
    pub fn run(&mut self) {
        if let Some(character) = self.characters.first_mut() {
            loop {
                let current_room = &self.rooms[character.position()];

                println!("\n🌍 {} est actuellement dans : {}", character.name(), current_room.name);
                println!("📍 {} : {}", current_room.name, current_room.description);

                // Affichage des objets trouvés
                if !current_room.items.is_empty() {
                    println!("🛠 Objets trouvés :");
                    for &item_id in &current_room.items {
                        if let Some(item) = self.items.iter().find(|i| i.id() == item_id) {
                            println!("- {} : {} (Effet : {})", item.name(), item.description(), item.effect().as_deref().unwrap_or("Aucun"));
                        }
                    }
                } else {
                    println!("🛠 Aucun objet trouvable ici.");
                }

                // Affichage des PNJ présents
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

                println!("\nOù veux-tu aller ? (north, south, east, west, quit, prendre <objet>, utiliser <objet>, parler <pnj>)");

                let mut input = String::new();
                stdin().read_line(&mut input).expect("Erreur de lecture");
                let input = input.trim().to_lowercase();

                if input == "quit" {
                    println!("🏁 Fin du jeu.");
                    break;
                }

                // Prendre un objet
                if input.starts_with("prendre ") {
                    let objet_nom = &input[8..].trim().to_lowercase(); // Normalisation
                
                    if let Some(&item_id) = self.rooms[character.position()]
                        .items
                        .iter()
                        .find(|&&id| self.items[id as usize].name().to_lowercase() == *objet_nom)
                    {
                        let item = self.items[item_id as usize].clone();
                        character.inventory().push(item.clone());
                        println!("🎒 {} a pris l'objet : {}", character.name(), item.name());
                
                        // Supprimer l'objet de la salle
                        self.rooms[character.position()].items.retain(|&id| id != item_id);
                    } else {
                        println!("❌ Objet non trouvé !");
                    }
                    continue;
                }
                
                
                // Parler à un PNJ
                if input.starts_with("parler ") {
                    let pnj_nom = &input[7..].trim();
                    Pnj::parler_au_pnj(pnj_nom, character.position(), &self.rooms, &self.pnjs, &self.dialogues);
                    continue;
                }
                
                

                // Déplacement du personnage
                if let Some(&new_position) = current_room.exits.get(&input) {
                    character.set_position(new_position);
                    println!("🚶 {} se déplace vers {}.", character.name(), self.rooms[new_position].name);
                } else {
                    println!("❌ Pas de passage dans cette direction !");
                }
            }
        }
    }
}
