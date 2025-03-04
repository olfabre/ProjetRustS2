mod character;
mod combat;
mod event;
mod io_handler;
mod npc;
mod world;
mod data_loader;

mod item;

use character::Character;
use combat::Combat;
use event::Event;
use io_handler::talk_to_npc;
use world::World;

fn main() {
    println!("🎮 Bienvenue dans l'aventure RPG !");

    // Charger le joueur depuis le fichier JSON
    let mut player = Character::load_from_json();
    player.display();

    // Charger le monde
    let world = World::load_from_json();

    loop {
        println!("\nQue voulez-vous faire ?");
        println!("1. Explorer");
        println!("2. Parler à un PNJ");
        println!("3. Combattre un ennemi");
        println!("4. Voir mes quêtes");
        println!("5. Voir mon inventaire");
        println!("6. Quitter");

        let choice = io_handler::get_user_input();
        match choice.trim() {
            "1" => {
                world.explore(&mut player);
                Event::random_event(&mut player);
            }
            "2" => {
                talk_to_npc(&mut player);
            }
            "3" => {
                Combat::fight(&mut player);
            }
            "4" => {
                player.check_quests();
            }
            "5" => {
                player.check_inventory();
            }
            "6" => {
                println!("👋 Merci d'avoir joué !");
                break;
            }
            _ => println!("❌ Choix invalide, essayez encore.")
        }
    }
}
