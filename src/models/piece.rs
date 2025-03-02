
use serde::{Serialize, Deserialize};
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
