// Gestion des entrées/sorties (interaction utilisateur)
use std::io::{self, Write};
use crate::game::character::Character;
use crate::game::world::World;
use crate::game::data_loader::load_game_data;

pub fn get_user_input() -> String {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Erreur de lecture.");
    input.trim().to_string()
}

pub fn talk_to_npc(player: &mut Character) {
    let data = load_game_data();
    let world = data.world;

    println!("Dans quelle zone voulez-vous parler à un PNJ ?");
    for (i, zone) in world.zones.iter().enumerate() {
        println!("{}. {}", i + 1, zone.name);
    }

    let choice = get_user_input().parse::<usize>().unwrap_or(0);
    if choice == 0 || choice > world.zones.len() {
        println!("❌ Choix invalide.");
        return;
    }

    let zone = &world.zones[choice - 1];

    if zone.npcs.is_empty() {
        println!("Il n'y a personne ici...");
        return;
    }

    println!("Avec qui voulez-vous parler ?");
    for (i, npc) in zone.npcs.iter().enumerate() {
        println!("{}. {}", i + 1, npc.name);
    }

    let npc_choice = get_user_input().parse::<usize>().unwrap_or(0);
    if npc_choice == 0 || npc_choice > zone.npcs.len() {
        println!("❌ Choix invalide.");
        return;
    }

    let npc = &zone.npcs[npc_choice - 1];
    println!("👤 {} : \"{}\"", npc.name, npc.dialogue[0]);

    if let Some(quest) = npc.quests.first() {
        println!("🔹 {} a une quête pour vous : {}", npc.name, quest.name);
        println!("   📜 Description : {}", quest.description);
        println!("   🎁 Récompense : {}", quest.reward);
        println!("Voulez-vous accepter cette quête ? (oui/non)");

        let response = get_user_input();
        if response.trim().to_lowercase() == "oui" {
            player.accept_quest(quest.clone());
        }
    }
}
