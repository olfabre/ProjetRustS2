mod models;
mod io;
use models::traits::Descriptible;
use models::traits::Movable;
use models::traits::Interactable;
use std::io::stdin;


use io::loader::{load_room_from_file, load_characters_from_file, load_items_from_file};

fn main() {
    // Charger les salles
    let rooms = load_room_from_file("data/rooms.json").expect("Erreur lors du chargement des salles.");
    let mut characters = load_characters_from_file("data/characters.json").expect("Erreur lors du chargement des personnages.");
    let items = load_items_from_file("data/items.json").expect("Erreur lors du chargement des objets.");

    // Afficher les descriptions avec le trait
    println!("📍 Zones disponibles :");
    for room in &rooms {
        println!("{}\n{}\nType : {}", room.name, room.get_description(), room.terrain_type);
        if !room.items.is_empty() {
            println!("🌟 Objets trouvables : {:?}", room.items);
        }

        if !room.npcs.is_empty() {
            println!("🧑‍🤝‍🧑 PNJ présents : {:?}", room.npcs);
        }
    
        println!("---");
    }

    println!("\n🧑‍🎤 Personnages disponibles :");
    let characters_clone = characters.clone();

    for character in &characters_clone {
        println!("{}", character.get_description());

        
    }
    if let Some(character) = characters.first_mut() {
        loop {
            println!("\n🌍 {} est actuellement dans : {}", character.name, rooms[character.position].name);
            println!("Où veux-tu aller ? (north, south, east, west, quit)");

            // 🔹 Lire l'entrée utilisateur
            let mut direction = String::new();
            stdin().read_line(&mut direction).expect("Erreur de lecture");
            let direction = direction.trim().to_lowercase();

            if direction == "quit" {
                println!("🏁 Fin du déplacement.");
                break;
            }

            // 🔹 Déplacer le personnage
            character.move_to(&direction, &rooms);
        }
    }

    println!("\n🛠 Objets disponibles :");
    for item in &items {
        println!("{}", item.get_description());
    }

    println!("🛠 Essayons d'interagir avec un objet...");
    if let Some(item) = items.first() {
        item.interact();
    }

    
}
