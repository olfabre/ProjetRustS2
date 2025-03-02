

pub trait GameObject {
    fn get_type(&self) -> &str;
    fn afficher(&self);
    fn decriver(&self);
}
