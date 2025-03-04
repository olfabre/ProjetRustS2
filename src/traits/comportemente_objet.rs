
use crate::models::Piece;

pub trait ComportementObjet {
    fn get_type(&self) -> &str;
    fn decriver(&self);
}
