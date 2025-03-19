use serde::{Deserialize, Serialize};

// STRUCT DE BASE
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Element {
    id: u32,
    name: String,
    description: String,
}

impl Element {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}