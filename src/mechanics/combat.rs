use crate::traits::combatant::Combatant;

/// Gère un tour de combat entre deux entités
pub fn run_combat_round(attacker: &dyn Combatant, defender: &mut dyn Combatant) {
    if attacker.is_alive() && defender.is_alive() {
        attacker.attack(defender);
    }

    if !defender.is_alive() {
        println!("{} est vaincu !", defender.get_name());
    }
}

/// Combat au tour par tour jusqu’à ce qu’un des deux meure
pub fn run_combat(mut entity1: Box<dyn Combatant>, mut entity2: Box<dyn Combatant>) {
    println!(
        "💥 Combat entre {} ({} PV) et {} ({} PV) !",
        entity1.get_name(),
        entity1.get_health(),
        entity2.get_name(),
        entity2.get_health()
    );

    let mut turn = 0;

    while entity1.is_alive() && entity2.is_alive() {
        println!("---- Tour {} ----", turn + 1);
        if turn % 2 == 0 {
            run_combat_round(&*entity1, &mut *entity2);
        } else {
            run_combat_round(&*entity2, &mut *entity1);
        }
        turn += 1;
    }

    println!("⚔️ Combat terminé !");
}
