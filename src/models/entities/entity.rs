use serde::{Deserialize, Serialize};

// STRUCT DE BASE
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    id: u32,
    name: String,
    description: String,
}




impl Entity {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }



}