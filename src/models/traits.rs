use crate::models::room::Room;

pub trait Descriptible {
    fn get_description(&self) -> String;
}

pub trait Interactable {
    fn interact(&self);
}

pub trait Movable {
    fn move_to(&mut self, direction: &str, rooms: &[Room]);
}
