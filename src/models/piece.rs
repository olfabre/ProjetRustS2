
use serde::{Serialize, Deserialize};
use crate::traits::{ComportementJoueur, ComportementObjet};
use super::connection::Connection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Piece {
    pub r#type: String,
    pub id: String,
    pub nom: String,
    pub description: String,
    pub connections: Vec<Connection>,
    pub inventaire: Vec<String>,
}

impl ComportementObjet for Piece {
    fn get_type(&self) -> &str {
        &self.r#type
    }

    fn decriver(&self) {
        println!(
            "{}",
            self.description
        );
    }
}