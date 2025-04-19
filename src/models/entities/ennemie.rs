
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Enemy {
    pub id: u32,
    pub name: String,
    pub health: i32,
    pub strength: i32,
    pub agility: i32,
    pub room_id: u32,
}


impl Enemy {
    pub fn new(id: u32, name: &str, health: i32, strength: i32, agility: i32, room_id: u32) -> Self{
        Self {
            id,
            name: name.to_string(),
            health,
            strength,
            agility,
            room_id,
        }
    }

    pub fn is_alive(&self) -> bool{

        self.health > 0
    }
}