use crate::models::entities::room::Room;

pub trait Descriptible {
    fn get_description(&self) -> String;
}

pub trait Interactable {
    fn interact(&self);
}

pub trait Movable {
    fn move_to_position(&mut self, direction: usize);
    fn get_position(&self);
}

