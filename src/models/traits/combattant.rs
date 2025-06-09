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

// Trait définissant les capacités de combat pour les entités du jeu
// Permet aux entités de s'engager dans des combats avec d'autres entités
pub trait Combattant: std::fmt::Debug {
    // Retourne le nom du combattant
    fn nom(&self) -> &str;
    // Retourne la force du combattant (utilisée pour les calculs de dégâts)
    fn force(&self) -> u32;
    // Retourne les points de vie actuels du combattant
    fn sante(&self) -> u32;
    // Vérifie si le combattant est toujours en vie
    fn est_vivant(&self) -> bool;

    // Inflige des dégâts au combattant
    // degats : le montant de dégâts à infliger
    fn infliger_degats(&mut self, degats: u32);
    // Calcule les dégâts de base de l'attaque
    fn degats_attaque(&self) -> u32;
    // Calcule la protection contre les dégâts
    fn protection_defense(&self) -> u32;

    // Effectue une attaque contre un autre combattant
    // autre : le combattant cible de l'attaque
    // Retourne le résultat du combat (victoire ou combat en cours)
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

// Exécute un tour de combat entre deux combattants
// attacker : le combattant qui attaque
// defender : le combattant qui défend
pub fn run_combat_round(attacker: &Box<dyn Combattant>, defender: &mut Box<dyn Combattant>) {
    if attacker.est_vivant() && defender.est_vivant() {
        attacker.as_ref().attaquer(defender.as_mut());
    }

    if !defender.est_vivant() {
        println!("{} est vaincu !", defender.nom());
    }
}

// Exécute un combat complet entre deux entités
// Les combattants s'attaquent à tour de rôle jusqu'à ce qu'un des deux soit vaincu
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

// Énumération des résultats possibles d'un combat
pub enum CombatResult {
    VICTORY,  // Le combattant a remporté le combat
    DEFEAT,   // Le combattant a perdu le combat
    ONGOING,  // Le combat est toujours en cours
    Ongoing,  // Alias pour ONGOING (à nettoyer)
}
