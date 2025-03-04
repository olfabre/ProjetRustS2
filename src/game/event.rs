use rand::Rng;
use crate::game::{character::Character, item::{Item, ItemType}, enemy::Enemy, combat::Combat};

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
        let event_type = rng.gen_range(0..3); // 3 types d'événements possibles

        match event_type {
            0 => { // Trésor trouvé
                let treasure = Item::new("Bijou ancien", ItemType::Potion, 50);
                println!("💰 Vous avez trouvé un coffre contenant un trésor : {} !", treasure.name);
                player.add_item(treasure);
            }
            1 => { // Attaque ennemie - Mais uniquement si le joueur **n'est pas déjà en combat**
                if player.health > 0 {
                    let enemy = Enemy::new("Bandit", 30, 7, 5); // Vérification pour éviter un combat en double
                    println!("⚠️ Un {} surgit de nulle part et vous attaque !", enemy.name);
                    Combat::fight(player);
                }
            }
            2 => { // Rencontre avec un PNJ
                println!("👤 Un voyageur mystérieux vous arrête et vous donne un objet...");
                let gift = Item::new("Potion secrète", ItemType::Potion, 30);
                player.add_item(gift);
            }
            _ => (),
        }
    }
}