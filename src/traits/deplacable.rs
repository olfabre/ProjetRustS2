
use crate::models::{Piece, Oridentation};

pub trait Deplacable {
    fn avancer(&mut self, pieces: &Vec<Piece>, direction: Oridentation);


}
