
/*pub trait Combattant {
    fn get_name(&self) -> &str;
    fn get_health(&self) -> i32;
    fn get_strength(&self) -> i32;

    fn receive_damage(&mut self, amount: i32);

    fn is_alive(&self) -> bool {
        self.get_health() > 0
    }

    fn attack(&self, target: &mut dyn Combattant) {
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

fn run_combat_round(attacker: &dyn Combattant, defender: &mut dyn Combattant) {
    if attacker.is_alive() && defender.is_alive() {
        attacker.attack(defender);
    }

    if !defender.is_alive() {
        std::println!("{} est vaincu !", defender.get_name());
    }
}

/// Combat au tour par tour jusqu’à ce qu’un des deux meure

fn run_combat(mut entity1: Box<dyn Combattant>, mut entity2: Box<dyn Combattant>) {
    std::println!(
        "💥 Combat entre {} ({} PV) et {} ({} PV) !",
        entity1.get_name(),
        entity1.get_health(),
        entity2.get_name(),
        entity2.get_health()
    );

    let mut turn = 0;

    while entity1.is_alive() && entity2.is_alive() {
        std::println!("---- Tour {} ----", turn + 1);
        if turn % 2 == 0 {
            run_combat_round(&*entity1, &mut *entity2);
        } else {
            run_combat_round(&*entity2, &mut *entity1);
        }
        turn += 1;
    }

    std::println!("⚔️ Combat terminé !");
}

pub enum CombatResult {
    VICTORY,
    DEFEAT,
    ONGOING,
    Ongoing,
}
