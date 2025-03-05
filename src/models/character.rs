use serde::{Deserialize, Serialize};
use crate::models::traits::Movable;
use crate::models::traits::Descriptible;
use crate::models::room::Room;
use crate::models::item::Item;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub id: u32,
    pub name: String,
    pub health: i32,
    pub strength: i32,
    pub intelligence: i32,
    pub position: usize,
    pub inventory: Vec<Item>,
}

impl Descriptible for Character {
    fn get_description(&self) -> String {
        format!(
            "{} (Santé: {}, Force: {}, Intelligence: {})",
            self.name, self.health, self.strength, self.intelligence
        )
    }
}

impl Movable for Character {
    fn move_to(&mut self, direction: &str, rooms: &[Room]) {
        let current_room = &rooms[self.position as usize];

        match direction {
            "north" => {
                if let Some(room_id) = current_room.north {
                    self.position = room_id as usize;
                    println!("{} se déplace vers le nord.", self.name);
                } else {
                    println!("Impossible d'aller au nord.");
                }
            }
            "south" => {
                if let Some(room_id) = current_room.south {
                    self.position = room_id as usize;
                    println!("{} se déplace vers le sud.", self.name);
                } else {
                    println!("Impossible d'aller au sud.");
                }
            }
            "east" => {
                if let Some(room_id) = current_room.east {
                    self.position = room_id as usize;
                    println!("{} se déplace vers l'est.", self.name);
                } else {
                    println!("Impossible d'aller à l'est.");
                }
            }
            "west" => {
                if let Some(room_id) = current_room.west {
                    self.position = room_id as usize;
                    println!("{} se déplace vers l'ouest.", self.name);
                } else {
                    println!("Impossible d'aller à l'ouest.");
                }
            }
            _ => println!("Commande invalide."),
        }
    }
}