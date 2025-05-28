
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
