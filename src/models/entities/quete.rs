use serde::{Deserialize, Serialize};
use crate::models::dialogue::DialogueStep;
use crate::models::entities::entity::Entity;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quete {
    ent: Entity,
    statu: String, // Pas accepté, en cours, terminé
    pnj_giver: u32,
    pnj_complete: u32,
    objectif: u32,
    recompense: u32,

}

impl Quete {



    pub fn id(&self) -> u32 {
        self.ent.id()
    }

    pub fn name(&self) -> &str {
        self.ent.name()
    }


}