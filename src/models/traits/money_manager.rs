use crate::models::entities::character::Character;
use crate::models::entities::pnj::Pnj;

pub trait MoneyManager {

    fn money_mut(&mut self) -> &mut i32;

    fn add_money(&mut self, amount: i32) {
        *self.money_mut() += amount;
    }

    fn remove_money(&mut self, amount: i32) {
        if amount <= *self.money_mut() {
            *self.money_mut() -= amount;
        } else {
            println!("❌ Pas assez d'argent !");
        }
    }
}

impl MoneyManager for Character {
    fn money_mut(&mut self) -> &mut i32 {
        &mut self.money
    }
}

impl MoneyManager for Pnj {
    fn money_mut(&mut self) -> &mut i32 {
        &mut self.money
    }
}