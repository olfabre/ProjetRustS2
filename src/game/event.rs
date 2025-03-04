use rand::Rng;
use crate::game::{character::Character, item::Item, enemy::Enemy, combat::Combat};
use crate::game::data_loader::load_game_data;
use rand::seq::SliceRandom;

pub enum EventType {
    Treasure,
    EnemyAttack,
    NpcEncounter,
}

pub struct Event {
    pub event_type: EventType,
    pub description: String,
}

impl Event {
    pub fn random_event(player: &mut Character) {
        let mut rng = rand::thread_rng();
        let data = load_game_data();
        let event_type = rng.gen_range(0..3); // 3 types d'événements possibles

        match event_type {
            0 => { // 🎁 Trésor trouvé (objet aléatoire)
                if let Some(treasure) = data.items.choose(&mut rng) {
                    println!("💰 Vous avez trouvé un coffre contenant un trésor : {} !", treasure.name);
                    player.add_item(treasure.clone());
                } else {
                    println!("🚫 Aucun trésor trouvé...");
                }
            }
            1 => { // ⚔️ Attaque ennemie
                if let Some(enemy) = data.enemies.choose(&mut rng) {
                    println!("⚠️ Un {} surgit de nulle part et vous attaque !", enemy.name);
                    let mut enemy_clone = enemy.clone();
                    Combat::fight(player);
                } else {
                    println!("🚫 Aucun ennemi trouvé...");
                }
            }
            2 => { // 🗣️ Rencontre avec un PNJ
                let npcs: Vec<_> = data.world.zones.iter().flat_map(|zone| &zone.npcs).collect();
                if let Some(npc) = npcs.choose(&mut rng) {
                    println!("👤 Vous rencontrez {} : \"{}\"", npc.name, npc.dialogue[0]);

                    if let Some(quest) = npc.quests.first() {
                        println!("🔹 {} a une quête pour vous : {}", npc.name, quest.name);
                        println!("   📜 Description : {}", quest.description);
                        println!("   🎁 Récompense : {}", quest.reward);
                        println!("Voulez-vous accepter cette quête ? (oui/non)");

                        let response = crate::game::io_handler::get_user_input();
                        if response.trim().to_lowercase() == "oui" {
                            player.accept_quest(quest.clone());
                        }
                    }
                } else {
                    println!("🚫 Aucun PNJ rencontré...");
                }
            }
            _ => (),
        }
    }
}
