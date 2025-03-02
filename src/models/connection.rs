
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]

pub enum Oridentation {
    NORD,
    EST,
    SUD,
    OUEST,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub orientation: Oridentation,
    pub destination: String,
}
