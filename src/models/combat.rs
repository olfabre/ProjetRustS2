use std::io;
use std::io::Write;
use crate::models::{entities::character::Character, entities::ennemie::Enemy}; // Import du joueur et des ennemis
use rand::Rng; // Pour générer des événements aléatoires (ex : attaque spéciale ennemie)
use crate::io::loader::get_user_input; // Fonction utilitaire pour lire l'entrée utilisateur

/// Structure vide pour regrouper les fonctions liées au combat
pub struct Combat;

impl Combat {
    /// Lance un combat entre le joueur et un ennemi
    pub fn fight(player: &mut Character, mut enemy: Enemy) -> bool {
        println!("⚔️ Un {} sauvage apparaît dans la salle !", enemy.name);

        loop {
            println!("\n👤 Vos PV : {} | 👹 PV de {} : {}", player.health, enemy.name, enemy.health);
            println!("🎮 Que voulez-vous faire ?");
            println!("1. Attaquer");
            println!("2. Attaque spéciale");
            println!("3. Fuir");

            let choice = get_user_input();

            match choice.trim() {
                "1" => {
                    let damage = player.strength;
                    enemy.health = enemy.health.saturating_sub(damage).try_into().unwrap();
                    println!("🗡️ Vous infligez {} dégâts à {}.", damage, enemy.name);
                }
                "2" => {
                    let special_damage = player.strength + 5;
                    enemy.health = enemy.health.saturating_sub(special_damage);
                    println!("💥 Attaque spéciale ! Vous infligez {} dégâts à {}.", special_damage, enemy.name);
                }
                "3" => {
                    println!("🏃 Vous fuyez le combat !");
                    return false; // l'ennemi n'est pas mort
                }
                _ => {
                    println!("❌ Choix invalide !");
                    continue;
                }
            }

            if !enemy.is_alive() {
                println!("🎉 Vous avez vaincu {} !", enemy.name);
                player.add_experience(50);
                return true; // <- L’ennemi est vaincu
            }

            // Tour de l'ennemi
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.3) {
                println!("🔥 {} utilise une attaque spéciale !", enemy.name);
                player.health = player.health.saturating_sub(10);
                println!("💔 Vous perdez 10 points de vie !");
            } else {
                println!("🔁 {} riposte !", enemy.name);
                player.health = player.health.saturating_sub(enemy.strength);
                println!("💔 Vous perdez {} points de vie !", enemy.strength);
            }

            if player.health == 0 {
                println!("💀 Vous êtes mort...");
                return false; // <- le joueur est mort
            }
        }
    }

}
