// Module de gestion du système de combat
// Ce module contient les fonctions et structures nécessaires pour gérer les combats entre entités

use std::io;
use std::io::Write;
use crate::models::{entities::character::Character, entities::Enemy::Enemy}; // Import du joueur et des ennemis
use rand::Rng; // Pour générer des événements aléatoires (ex : attaque spéciale ennemie)
use crate::io::loader::get_user_input;
use crate::models::traits::combattant::Combattant;
// Fonction utilitaire pour lire l'entrée utilisateur

// Structure vide pour regrouper les fonctions liées au combat
// Cette structure était utilisée pour implémenter le système de combat basé sur des choix
// Elle a été remplacée par un système plus flexible utilisant le trait Combattant
// pub trait Combat {
//     /// Lance un combat entre le joueur et un ennemi
//     /// Permet au joueur de choisir ses actions (attaque normale, spéciale ou fuite)
//     /// Gère les dégâts et les effets spéciaux des ennemis
//     /// Retourne true si l'ennemi est vaincu, false si le joueur fuit ou meurt
//     fn fight(player: &mut Character, mut enemy: Enemy) -> bool {
//         println!("⚔️ Un {} sauvage apparaît dans la salle !");
//
//         loop {
//             println!("\n👤 Vos PV : {} | 👹 PV de {} : {}", player.health(), enemy.name(), enemy.health());
//             println!("🎮 Que voulez-vous faire ?");
//             println!("1. Attaquer");
//             println!("2. Attaque spéciale");
//             println!("3. Fuir");
//
//             let choice = get_user_input();
//
//             match choice.trim() {
//                 "1" => {
//                     let damage = player.strength();
//
//                     enemy.set_health(enemy.health().saturating_sub(damage));
//
//                     println!("🗡️ Vous infligez {} dégâts à {}.", damage, enemy.name());
//                 }
//                 "2" => {
//                     let special_damage = player.strength() + 5;
//                     enemy.set_health(enemy.health().saturating_sub(special_damage));
//                     println!("💥 Attaque spéciale ! Vous infligez {} dégâts à {}.", special_damage, enemy.name());
//                 }
//                 "3" => {
//                     println!("🏃 Vous fuyez le combat !");
//                     return false; // l'ennemi n'est pas mort
//                 }
//                 _ => {
//                     println!("❌ Choix invalide !");
//                     continue;
//                 }
//             }
//
//             if !enemy.is_alive() {
//                 println!("🎉 Vous avez vaincu {} !", enemy.name());
//                 player.add_experience(50);
//                 return true; // <- L'ennemi est vaincu
//             }
//
//             // Tour de l'ennemi
//             let mut rng = rand::thread_rng();
//             if rng.gen_bool(0.3) {
//                 println!("🔥 {} utilise une attaque spéciale !", enemy.name());
//                 player.set_health(player.health().saturating_sub(10));
//                 println!("💔 Vous perdez 10 points de vie !");
//             } else {
//                 println!("🔁 {} riposte !", enemy.name());
//                 player.set_health(player.health().saturating_sub(enemy.strength()));
//                 println!("💔 Vous perdez {} points de vie !", enemy.strength());
//             }
//
//             if player.health() == 0 {
//                 println!("💀 Vous êtes mort...");
//                 return false; // <- le joueur est mort
//             }
//         }
//     }
// }

// Les fonctions de combat ont été déplacées dans le module traits::combattant
// pour une meilleure organisation et réutilisation du code

// Gère un tour de combat entre deux entités
/*pub fn run_combat_round(attacker: &dyn Combattant, defender: &mut dyn Combattant) {
    if attacker.is_alive() && defender.is_alive() {
        attacker.attack(defender);
    }

    if !defender.is_alive() {
        println!("{} est vaincu !", defender.get_name());
    }
}

// Combat au tour par tour jusqu'à ce qu'un des deux meure
pub fn run_combat(mut entity1: Box<dyn Combattant>, mut entity2: Box<dyn Combattant>) {
    println!(
        "💥 Combat entre {} ({} PV) et {} ({} PV) !",
        entity1.get_name(),
        entity1.get_health(),
        entity2.get_name(),
        entity2.get_health()
    );

    let mut turn = 0;

    while entity1.is_alive() && entity2.is_alive() {
        println!("---- Tour {} ----", turn + 1);
        if turn % 2 == 0 {
            run_combat_round(&*entity1, &mut *entity2);
        } else {
            run_combat_round(&*entity2, &mut *entity1);
        }
        turn += 1;
    }

    println!("⚔️ Combat terminé !");
}*/