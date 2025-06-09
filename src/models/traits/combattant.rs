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
use crate::models::entities::loot_entry::LootEntry;

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


    fn loot(&self) -> &[LootEntry];
    fn experience_gain(&self) -> i32;



}

// Énumération des résultats possibles d'un combat
pub enum CombatResult {
    VICTORY,  // Le combattant a remporté le combat
    DEFEAT,   // Le combattant a perdu le combat
    ONGOING,  // Le combat est toujours en cours
    Ongoing,  // Alias pour ONGOING (à nettoyer)
}
