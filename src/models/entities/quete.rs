use serde::{Deserialize, Serialize};
use crate::models::dialogue::DialogueStep;
use crate::models::entities::entity::Entity;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quete {
    entity: Entity,
    statu: String, // disponible, accepté, terminé
    // pnj_giver: u32,
    // pnj_complete: u32,
    objectif: u32,
    recompense: Vec<String>,
    recompense_items: Vec<u32>,
    recompense_argent: i32,

}

impl Quete {



    pub fn id(&self) -> u32 {
        self.entity.id()
    }

    pub fn name(&self) -> &str {
        self.entity.name()
    }


}