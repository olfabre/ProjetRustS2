// Gestion des entrées/sorties (interaction utilisateur)
use std::io::{self, Write};
use crate::game::character::Character;
use crate::game::world::World;








pub fn get_user_input() -> String {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Erreur de lecture.");
    input
}

pub fn talk_to_npc(world: &World, player_location: &str, player: &mut Character) {
    let zone = world.zones.iter().find(|z| z.name == player_location);
    if let Some(zone) = zone {
        if zone.npcs.is_empty() {
            println!("Il n'y a personne ici...");
            return;
        }

        println!("Avec qui voulez-vous parler ?");
        for (i, npc) in zone.npcs.iter().enumerate() {
            println!("{}. {}", i + 1, npc.name);
        }

        let choice = super::io_handler::get_user_input().trim().parse::<usize>().unwrap_or(0);
        if choice > 0 && choice <= zone.npcs.len() {
            let npc = &zone.npcs[choice - 1];
            npc.talk(player);

            if !npc.quests.is_empty() {
                let quest = npc.quests[0].clone();
                println!("Voulez-vous accepter cette quête ? (oui/non)");
                let response = super::io_handler::get_user_input();
                if response.trim().to_lowercase() == "oui" {
                    player.accept_quest(quest);
                }
            }
        } else {
            println!("Choix invalide.");
        }
    } else {
        println!("Lieu inconnu...");
    }
}

