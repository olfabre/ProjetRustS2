// Système de combat

use crate::game::{character::Character, enemy::Enemy};
use crate::game::io_handler::get_user_input;
use rand::Rng;

pub struct Combat; 

impl Combat {

    pub fn fight(player: &mut Character) {
        let mut enemy = Enemy::new("Gobelin", 20, 5, 3);
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
    
            /*if enemy.is_alive() {
                println!("{} riposte !", enemy.name);
                player.health -= enemy.strength;
                println!("Vous perdez {} points de vie.", enemy.strength);
            } else {
                println!("🎉 Vous avez vaincu {} !", enemy.name);
                return;
            }*/

            if enemy.is_alive() {
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.3) { // 30% de chances d’un événement en combat
                    println!("🔥 L'ennemi déclenche une attaque spéciale !");
                    player.health -= 10;
                    println!("Vous perdez 10 points de vie !");
                } else {
                    println!("{} riposte !", enemy.name);
                    player.health -= enemy.strength;
                    println!("Vous perdez {} points de vie.", enemy.strength);
                }
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
