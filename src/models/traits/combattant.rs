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

// Trait qui définit les capacités de combat d'une entité
// Permet de gérer les attaques, la défense et les points de vie
pub trait Combattant: std::fmt::Debug {
    // Retourne le nom du combattant
    fn nom(&self) -> &str;

    // Retourne la force d'attaque du combattant
    fn force(&self) -> u32;

    // Retourne les points de vie actuels du combattant
    fn sante(&self) -> u32;

    // Vérifie si le combattant est en vie
    fn est_vivant(&self) -> bool;

    // Applique des dégâts au combattant
    fn infliger_degats(&mut self, degats: u32);

    // Calcule les dégâts d'attaque du combattant
    fn degats_attaque(&self) -> u32;

    // Calcule la protection défensive du combattant
    fn protection_defense(&self) -> u32;

    // Effectue une attaque sur un autre combattant
    // Retourne le résultat du combat (victoire ou en cours)
    fn attaquer(&self, autre: &mut dyn Combattant) -> CombatResult {
        // Calcule les dégâts en tenant compte de la force et de la protection
        let degats_bruts = self.degats_attaque() + self.force();
        let protection = autre.protection_defense();
        let degats_reels = degats_bruts.saturating_sub(protection);

        // Applique les dégâts au défenseur
        autre.infliger_degats(degats_reels);

        // Vérifie si le défenseur est mort
        if !autre.est_vivant() {
            CombatResult::VICTORY
        } else {
            CombatResult::ONGOING
        }
    }
}

// Exécute un tour de combat entre deux combattants
pub fn run_combat_round(attacker: &Box<dyn Combattant>, defender: &mut Box<dyn Combattant>) {
    // Vérifie que les deux combattants sont en vie
    if attacker.est_vivant() && defender.est_vivant() {
        attacker.as_ref().attaquer(defender.as_mut());
    }

    // Affiche un message si le défenseur est vaincu
    if !defender.est_vivant() {
        println!("{} est vaincu !", defender.nom());
    }
}

// Gère un combat complet entre deux entités
// Les combattants s'affrontent à tour de rôle jusqu'à ce qu'un des deux meure
pub fn run_combat(mut entity1: Box<dyn Combattant>, mut entity2: Box<dyn Combattant>) {
    // Affiche les informations initiales du combat
    println!(
        "💥 Combat entre {} ({} PV) et {} ({} PV) !",
        entity1.nom(),
        entity1.sante(),
        entity2.nom(),
        entity2.sante()
    );

    let mut turn = 0;

    // Boucle de combat jusqu'à ce qu'un des combattants meure
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

// Énumération des résultats possibles d'un combat
pub enum CombatResult {
    VICTORY,    // Victoire du combattant
    DEFEAT,     // Défaite du combattant
    ONGOING,    // Combat en cours
    Ongoing,    // Alias pour ONGOING (à supprimer)
}
