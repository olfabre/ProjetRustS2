
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

pub trait Combattant: std::fmt::Debug {

    fn nom(&self) -> &str;
    fn force(&self) -> u32;
    fn sante(&self) -> u32;
    fn est_vivant(&self) -> bool;

    fn infliger_degats(&mut self, degats: u32);
    fn degats_attaque(&self) -> u32;
    fn protection_defense(&self) -> u32;

    fn attaquer(&self, autre: &mut dyn Combattant) -> CombatResult {
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


pub fn run_combat_round(attacker: &Box<dyn Combattant>, defender: &mut Box<dyn Combattant>) {
    if attacker.est_vivant() && defender.est_vivant() {
        attacker.as_ref().attaquer(defender.as_mut());

    }

    if !defender.est_vivant() {
        println!("{} est vaincu !", defender.nom());
    }
}

// Combat au tour par tour jusqu’à ce qu’un des deux meure
pub fn run_combat(mut entity1: Box<dyn Combattant>, mut entity2: Box<dyn Combattant>) {
    println!(
        "💥 Combat entre {} ({} PV) et {} ({} PV) !",
        entity1.nom(),
        entity1.sante(),
        entity2.nom(),
        entity2.sante()
    );

    let mut turn = 0;

    while entity1.est_vivant() && entity2.est_vivant() {
        println!("---- Tour {} ----", turn + 1);
        if turn % 2 == 0 {
            run_combat_round(&entity1, &mut entity2);
        } else {
            run_combat_round(&entity2, &mut entity1);
        }
        turn += 1;
    }

    println!("⚔️ Combat terminé !");
}


pub enum CombatResult {
    VICTORY,
    DEFEAT,
    ONGOING,
    Ongoing,
}
