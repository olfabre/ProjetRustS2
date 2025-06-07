use std::collections::HashMap;
use crate::models::entities::character::Character;
use crate::models::entities::Enemy::Enemy;
use crate::models::entities::item::Item;
use crate::models::entities::quete::Quete;

pub trait QueteManager {
    fn ajouter_quete(&mut self, id: u32);
    fn supprimer_quete(&mut self, id: u32);
    fn get_active_quests(&self, all_quests: &HashMap<u32, Quete>, items: &Vec<Item>, enemies: &HashMap<u32, Enemy>) -> Vec<String>;
}

impl QueteManager for Character {

    fn ajouter_quete(&mut self, id: u32) {
        self.quests.push(id);
    }

    fn supprimer_quete(&mut self, id: u32) {
        self.quests.retain(|&q| q != id);
    }

    /// Renvoie une liste des descriptions de quêtes actuellement actives pour le personnage.
    /// Associe les identifiants de quête aux données de quête et formate l'objectif pour l'affichage.
    fn get_active_quests(&self, all_quests: &HashMap<u32, Quete>, items: &Vec<Item>, enemies: &HashMap<u32, Enemy>) -> Vec<String> {
        // Créez un vecteur pour stocker les descriptions des quêtes correspondantes.
        let mut quest_titles: Vec<String> = vec![];

        // Itérer sur chaque ID de quête dans la liste des quêtes du personnage.
        for &quest_id_from_char in &self.quests {
            // Initialise une chaîne de caractères pour stocker la description formatée de la quête.
            let mut descriptor = String::from("* ");

            // Vérifie si la quête correspondant à l'ID existe dans la liste all_quests.
            if let Some(quest_found) = all_quests.get(&quest_id_from_char) {
                // Ajoute le nom et le type de l'objectif de la quête au descripteur.
                descriptor.push_str(&format!("{} - {}: ", quest_found.name(), quest_found.objectif_type));

                // Vérifie le type d'objectif de la quête.
                if quest_found.objectif_type == "collecter" {
                    // Recherche l'objet correspondant à l'ID de l'objectif de collecte.
                    let Some(item) = items.iter().find(|i| i.id() == quest_found.objectif.collecter.item_id) else { todo!() };

                    // Ajoute le nombre d'objets à collecter et leur nom au descripteur.
                    descriptor.push_str(&format!("{} - {}", quest_found.objectif.collecter.target, item.name()));
                } else if quest_found.objectif_type == "tuer" {
                    // Ajoute le nombre d'ennemis à éliminer et leur nom au descripteur.
                    descriptor.push_str(&format!("{} - {}", quest_found.objectif.tuer.target, enemies.get(&quest_found.objectif.tuer.ennemi_id).unwrap().name()));
                }
            }

            // Ajoute la description formatée de la quête au vecteur quest_titles.
            quest_titles.push(descriptor);
        }

        // Renvoie la liste des descriptions de quêtes formatées.
        quest_titles
    }

}