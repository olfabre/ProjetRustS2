// Trait de gestion de l'argent pour les entités du jeu
// Permet aux personnages et PNJ de gérer leur argent (achat, vente, récompenses)
pub trait MoneyManager {
    // Retourne une référence mutable à la quantité d'argent de l'entité
    // Utilisé pour les opérations de modification de l'argent
    fn money_mut(&mut self) -> &mut i32;

    // Ajoute une quantité d'argent à l'entité
    // Utilisé pour les gains, récompenses et ventes
    fn add_money(&mut self, amount: i32) {
        *self.money_mut() += amount;
    }

    // Retire une quantité d'argent de l'entité
    // Vérifie si l'entité a assez d'argent avant de retirer
    // Utilisé pour les achats et dépenses
    fn remove_money(&mut self, amount: i32) {
        if amount <= *self.money_mut() {
            *self.money_mut() -= amount;
        } else {
            println!("❌ Pas assez d'argent !");
        }
    }
}
