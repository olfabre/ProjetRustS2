use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub health: i32,
    pub strength: i32,
    pub agility: i32,
}

impl Enemy {
    pub fn new(name: &str, health: i32, strength: i32, agility: i32) -> Self {
        Self {
            name: name.to_string(),
            health,
            strength,
            agility,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
