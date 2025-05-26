
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

/*pub trait Combatant {
    fn get_name(&self) -> &str;
    fn get_health(&self) -> i32;
    fn get_strength(&self) -> i32;

    fn receive_damage(&mut self, amount: i32);

    fn take_damage(&mut self, amount: u32);

    fn is_alive(&self) -> bool {
        self.get_health() > 0
    }

    fn drop_loot(&self) -> Vec<InventoryItem> {
        vec![]
    }

    fn attack(&self, target: &mut dyn Combatant) {
        let damage = self.get_strength();
        println!("⚔️ {} attaque {} et inflige {} dégâts !", self.get_name(), target.get_name(), damage);
        target.receive_damage(damage);
    }

    fn combat(&mut self, enemy: &mut dyn Combatant) -> bool
    where
        Self: Sized,
    {
        println!("⚔️ Combat entre {} et {}", self.get_name(), enemy.get_name());

        while self.is_alive() && enemy.is_alive() {
            self.attack(enemy);
            if enemy.is_alive() {
                enemy.attack(self);
            }
        }

        if self.is_alive() {
            println!("🏆 {} a gagné !", self.get_name());
            true
        } else {
            println!("☠️ {} est tombé au combat.", self.get_name());
            false
        }
    }


}*/
