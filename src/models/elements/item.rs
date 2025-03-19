
use serde::{Deserialize, Serialize};
use crate::models::elements::element::Element;
use crate::models::traits::Descriptible;
use crate::models::traits::Interactable;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item { 
    elem: Element,
    effect: Option<String>,
    usable: bool,
}


impl Item {

    // GETTERS
    pub fn id(&self) -> u32 {
        self.elem.id()
    }

    pub fn name(&self) -> &str {
        &self.elem.name()
    }

    pub fn description(&self) -> &str {
        &self.elem.description()
    }

    pub fn effect(&self) -> &Option<String> {
        &self.effect
    }

    pub fn usable(&self) -> bool {
        self.usable
    }
}