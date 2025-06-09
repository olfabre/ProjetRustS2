use crate::models::entities::character::Character;

pub trait Combattant: std::fmt::Debug {

    fn nom(&self) -> &str;
    fn force(&self) -> u32;
    fn sante(&self) -> u32;
    fn est_vivant(&self) -> bool;

    fn infliger_degats(&mut self, degats: u32);
    fn degats_attaque(&self) -> u32;
    fn protection_defense(&self) -> u32;



}





pub enum CombatResult {
    VICTORY,
    DEFEAT,
    ONGOING,
    Ongoing,
}
