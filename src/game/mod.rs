 //Point d’entrée du module `game`
use crate::game::{combat::Combat, event::Event}; // ✅ Permet d'utiliser `Combat::fight()`
use rand::Rng;

pub mod character;
pub mod world;
pub mod inventory;
pub mod combat;
pub mod quest;
pub mod event;
pub mod io_handler;
pub mod save_load;
pub mod enemy;
pub mod item;
pub mod npc;
 mod data_loader;

 pub fn run() {
    println!("Bienvenue dans le RPG en mode texte !");
    
    let mut world = world::World::create_world();
    let player_location = "Village".to_string();
    let mut player = character::Character::new("Héros".to_string(), 10, 10, 10);
    let mut in_combat = false;

    loop {
        println!("\nQue voulez-vous faire ?");
        println!("1. Explorer");
        println!("2. Voir mon personnage");
        println!("3. Parler à un PNJ");
        println!("4. Voir mes quêtes");
        println!("5. Combattre un ennemi");
        println!("6. Utiliser un objet");
        println!("7. Voir mon inventaire");
        println!("8. Quitter");

        let choice = io_handler::get_user_input();
        match choice.trim() {
            "1" => world.explore(&mut player),
            "2" => player.display(),
            "3" => io_handler::talk_to_npc(&world, &player_location, &mut player),
            "4" => player.check_quests(),
            "5" => {
                if !in_combat {
                    in_combat = true;
                    Combat::fight(&mut player);
                    in_combat = false;
                }else {
                    println!("❌ Vous êtes déjà en combat !");
                }
                
            },
            "6" => player.use_item(),
            "7" => player.check_inventory(),
            "8" => {
                println!("Merci d'avoir joué !");
                break;
            }
            _ => println!("Choix invalide."),
        }

        // 🔥 Vérification avant de déclencher un événement aléatoire
        let mut rng = rand::thread_rng();
        if !in_combat && rng.gen_bool(0.3) { // Éviter les événements pendant un combat
            Event::random_event(&mut player);
        }
    }
}
