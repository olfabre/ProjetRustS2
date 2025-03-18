
pub struct Entity {
    id: u32,
    name: String,
}

impl Entity {
    fn new(id: u32, name: String) -> Entity {
        Self {
            id: id,
            name: name,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }


}