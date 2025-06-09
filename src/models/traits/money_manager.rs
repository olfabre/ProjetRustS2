// Trait définissant les capacités de gestion d'argent pour les entités du jeu
// Permet d'ajouter ou retirer de l'argent à une entité de manière sécurisée
pub trait MoneyManager {
    // Retourne une référence mutable vers le montant d'argent de l'entité
    fn money_mut(&mut self) -> &mut i32;

    // Ajoute un montant d'argent à l'entité
    // amount : le montant à ajouter (peut être négatif)
    fn add_money(&mut self, amount: i32) {
        *self.money_mut() += amount;
    }

    // Retire un montant d'argent de l'entité si elle en a assez
    // amount : le montant à retirer
    // Affiche un message d'erreur si l'entité n'a pas assez d'argent
    fn remove_money(&mut self, amount: i32) {
        if amount <= *self.money_mut() {
            *self.money_mut() -= amount;
        } else {
            println!("❌ Pas assez d'argent !");
        }
    }
}
