// Gestion du monde et des zones
use rand::Rng;
#[allow(unused_imports)]
//use crate::game::item::{Item, ItemType};
use crate::game::{character::Character, item::{Item, ItemType}, event::Event, quest::Quest};
use crate::game::npc::Npc;

pub enum ZoneEffect {
    None,
    Poisonous,     // Diminue la santé
    Healing,       // Régénère la santé
    StrengthBoost, // Augmente la force temporairement
}


pub struct Zone {
    pub name: String,
    pub description: String,
    pub npcs: Vec<Npc>,
    pub items: Vec<Item>,
    pub effect: ZoneEffect,
    pub requires_item: Option<String>,


}

pub struct World {
    pub zones: Vec<Zone>,
}


impl World {
    pub fn create_world() -> Self {
        Self {
            zones: vec![
                Zone {
                    name: "Forêt".to_string(),
                    description: "Une forêt sombre et inquiétante.".to_string(),
                    npcs: vec![],
                    items: vec![
                        Item::new("Amulette perdue", ItemType::Potion, 0), // Ajout de l'amulette
                        Item::new("Potion de soin", ItemType::Potion, 20),
                        Item::new("Épée en fer", ItemType::Weapon, 5),
                        Item::new("Épée rouillée", ItemType::Weapon, 3),
                    ],
                    effect: ZoneEffect::None,
                    requires_item: None,
                },
                Zone {
                    name: "Village".to_string(),
                    description: "Un village paisible.".to_string(),
                    npcs: vec![
                        Npc {
                            name: "Gérard".to_string(),
                            role: "Sage du village".to_string(),
                            dialogue: vec!["J’ai une mission spéciale pour toi...".to_string()],
                            quests: vec![Quest::new(
                                "Trouver l'amulette perdue",
                                "Retrouve l'amulette du sage cachée dans la forêt.",
                                "Clé dorée"
                            )],
                        },
                        Npc {
                            name: "Elena".to_string(),
                            role: "Aventurière".to_string(),
                            dialogue: vec!["J'ai une quête pour toi, es-tu intéressé ?".to_string()],
                            quests: vec![Quest::new("Sauver le village", "Protéger le village des monstres.", "100 pièces d'or")]
                        }
                    ],
                    items: vec![
                        Item::new("Armure légère", ItemType::Armor, 3),
                        Item::new("Bouclier en bois", ItemType::Armor, 2)
                    ],
                    effect: ZoneEffect::Healing,  // Régénère la santé
                    requires_item: None,
                    
                },

                Zone {
                    name: "Marais Toxique".to_string(),
                    description: "L'air est empoisonné et l'eau trouble...".to_string(),
                    items: vec![],
                    npcs: vec![],
                    effect: ZoneEffect::Poisonous,  // Diminue la santé
                    requires_item: Some("Clé dorée".to_string()),
                },

                Zone {
                    name: "Temple Mystique".to_string(),
                    description: "Une aura puissante émane de ces ruines anciennes...".to_string(),
                    items: vec![Item::new("Amulette magique", ItemType::Potion, 50)],
                    npcs: vec![],
                    effect: ZoneEffect::StrengthBoost,  // Augmente la force temporairement
                    requires_item: Some("Clé dorée".to_string()), // Nécessite un objet pour entrer
                },
            ],
        }
    }

    pub fn explore(&mut self, player: &mut crate::game::character::Character) {
        
        let mut rng = rand::thread_rng();
        let zone_index = rng.gen_range(0..self.zones.len());
        let zone = &mut self.zones[zone_index];

        // Vérifier si la zone nécessite un objet
        if let Some(ref required_item) = zone.requires_item {
            if !player.has_item(required_item) {
                println!("❌ Vous ne pouvez pas entrer dans {} sans {}", zone.name, required_item);
                return;
            }
        }

        println!("🌍 Vous arrivez dans : {}", zone.name);
        println!("{}", zone.description);
        
        // Effets aléatoires dans la zone
        /*if zone.name == "Forêt" {
            player.health -= 5;
            println!("Vous avez perdu 5 points de vie !");
        }*/

        match zone.effect {
            ZoneEffect::Poisonous => {
                player.health -= 10;
                println!("☠️ L'air toxique vous fait perdre 10 points de vie !");
            }
            ZoneEffect::Healing => {
                player.health += 10;
                println!("💖 L'énergie du village vous soigne de 10 points !");
            }
            ZoneEffect::StrengthBoost => {
                player.strength += 5;
                println!("💪 Une force mystique augmente votre puissance de 5 !");
            }
            _ => (),
        }

        // Ramasser des objets
        if !zone.items.is_empty() {
            println!("🔎 Vous trouvez des objets !");
            for item in zone.items.drain(..) {
                if !player.has_item(&item.name) {
                    player.add_item(item);
                } else {
                    println!("⚠️ Vous avez déjà {} dans votre inventaire !", item.name);
                }
            }
        }

        // Déclencher un événement aléatoire
        if rng.gen_bool(0.5) { // 50% de chances d’avoir un événement
            Event::random_event(player);
        }
    }
}
