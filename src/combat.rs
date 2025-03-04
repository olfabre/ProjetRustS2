// Système de combat

use crate::game::{character::Character, enemy::Enemy};
use crate::game::io_handler::get_user_input;
use crate::game::data_loader::load_game_data;
use rand::seq::SliceRandom;

pub struct Combat;

impl Combat {
    pub fn fight(player: &mut Character) {
        let data = load_game_data();
        let mut rng = rand::thread_rng();

        // Choisir un ennemi aléatoire depuis `game_data.json`
        let mut enemy = data.enemies.choose(&mut rng)
            .expect("Aucun ennemi trouvé dans le JSON")
            .clone();

        println!("Un {} sauvage apparaît !", enemy.name);

        loop {
            println!("\nQue voulez-vous faire ?");
            println!("1. Attaquer");
            println!("2. Attaque spéciale");
            println!("3. Fuir");

            let choice = get_user_input();
            match choice.trim() {
                "1" => player.attack(&mut enemy),
                "2" => player.special_attack(&mut enemy),
                "3" => {
                    println!("🏃 Vous fuyez le combat !");
                    return;
                }
                _ => println!("Choix invalide."),
            }

            if enemy.health > 0 {
                println!("{} riposte !", enemy.name);
                player.health -= enemy.strength;
                println!("Vous perdez {} points de vie.", enemy.strength);
            } else {
                println!("🎉 Vous avez vaincu {} !", enemy.name);
                player.add_experience(50);
                return;
            }

            if player.health <= 0 {
                println!("💀 Vous êtes mort...");
                return;
            }
        }
    }
}
