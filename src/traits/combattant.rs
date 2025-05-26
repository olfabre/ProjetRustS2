/*pub trait Combatant {
    fn get_name(&self) -> &str;
    fn get_health(&self) -> i32;
    fn get_strength(&self) -> i32;

    fn receive_damage(&mut self, amount: i32);

    fn is_alive(&self) -> bool {
        self.get_health() > 0
    }

    fn attack(&self, target: &mut dyn Combatant) {
        let damage = self.get_strength();
        println!(
            "{} attaque {} et inflige {} dégâts !",
            self.get_name(),
            target.get_name(),
            damage
        );
        target.receive_damage(damage);
    }
}*/

pub trait Combattant {
    fn nom(&self) -> &str;
    fn force(&self) -> u32;
    fn sante(&self) -> u32;
    fn est_vivant(&self) -> bool;

    fn infliger_degats(&mut self, degats: u32);
    fn degats_attaque(&self) -> u32;
    fn protection_defense(&self) -> u32;

    fn attaquer<C: Combattant>(&self, autre: &mut C) -> CombatResult {
        let degats_bruts = self.degats_attaque() + self.force();
        let protection = autre.protection_defense();
        let degats_reels = degats_bruts.saturating_sub(protection);

        autre.infliger_degats(degats_reels);

        if !autre.est_vivant() {
            CombatResult::VICTORY
        } else {
            CombatResult::ONGOING
        }
    }
}

pub enum CombatResult {
    VICTORY,
    DEFEAT,
    ONGOING,
    Ongoing,
}

