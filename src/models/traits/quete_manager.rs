use std::collections::HashMap;
use crate::models::dialogue::Dialogue;
use crate::models::entities::character::Character;
use crate::models::entities::quete::Quete;
use crate::models::game::Game;

pub trait QueteManager {
    fn ajouter_quete(&mut self, id: u32);
    fn supprimer_quete(&mut self, id: u32);
    fn get_active_quests(&self, game: &Game) -> Vec<String>;

    fn track_item(&mut self,
                  item_id: u32,
                  quetes: &mut HashMap<u32, Quete>,
                  dialogues: &mut Vec<Dialogue>) -> bool;

    fn track_enemy(&mut self,
                   ennemi_id: u32,
                   quetes: &mut HashMap<u32, Quete>,
                   dialogues: &mut Vec<Dialogue>);

    fn track_visit(&mut self,
                   room_id: u32,
                   quetes: &mut HashMap<u32, Quete>,
                   dialogues: &mut Vec<Dialogue>);

    /// Si une quête est terminée, les étapes de dialogue doivent changer pour que le joueur puisse
    /// rendre la quête
    fn update_dialogues(id: u32, dialogues: &mut Vec<Dialogue>) {
        // Rechercher le dialogue associé à l'ID donné
        let Some(mut dialogue) = dialogues.iter_mut().find(|mut dialog| {
            dialog.dialogue_id == id
        }) else { return };

        // Parcourez les étapes du dialogue et activez toute étape correspondant à l'achèvement d'une quête
        for step in &mut dialogue.dialogue_steps {
            if step.action.starts_with("rendreQuete") { // Vérifiez si l'action fait référence à l'achèvement d'une quête
                step.active = true; // Activez l'étape pour qu'elle fasse partie des options de dialogue du joueur
            }
        }
    }


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
    fn get_active_quests(&self, game: &Game) -> Vec<String> {
        // Créez un vecteur pour stocker les descriptions des quêtes correspondantes.
        let mut quest_titles: Vec<String> = vec![];

        // Itérer sur chaque ID de quête dans la liste des quêtes du personnage.
        for &quest_id_from_char in &self.quests {
            // Initialise une chaîne de caractères pour stocker la description formatée de la quête.
            let mut descriptor = String::from("* ");

            // Vérifie si la quête correspondant à l'ID existe dans la liste all_quests.
            if let Some(quest_found) = game.quetes.get(&quest_id_from_char) {
                // Ajoute le nom et le type de l'objectif de la quête au descripteur.
                descriptor.push_str(&format!("{} - {} ", quest_found.name(), quest_found.get_type()));

                // Vérifie le type d'objectif de la quête.
                if quest_found.get_type() == "collecter" {
                    // Recherche l'objet correspondant à l'ID de l'objectif de collecte.
                    let Some(item) = game.items.iter().find(|i| i.id() == quest_found.target_id()) else { todo!() };

                    // Ajoute le nombre d'objets à collecter et leur nom au descripteur.
                    descriptor.push_str(&format!("{}x  {}", quest_found.target(), item.name()));
                } else if quest_found.get_type() == "visiter"{
                    descriptor.push_str(&format!("la salle: {}",
                                                 game.rooms.iter().find(|room| room.id() == quest_found.target_id()).unwrap().name() ));
                } else if quest_found.get_type() == "tuer" {
                    // Ajoute le nombre d'ennemis à éliminer et leur nom au descripteur.
                    descriptor.push_str(&format!("{}x  {}", quest_found.target(), game.enemies.get(&quest_found.target_id()).unwrap().name()));
                }
            }

            // Ajoute la description formatée de la quête au vecteur quest_titles.
            quest_titles.push(descriptor);
        }

        // Renvoie la liste des descriptions de quêtes formatées.
        quest_titles
    }

    /// Lors de l'ajout d'un objet à l'inventaire, cette fonction est appelée et vérifie si l'objet
    /// est un objet nécessaire pour terminer une quête
    /// Renvoie true ou false
    fn track_item(&mut self,
                  item_id: u32,

                  quetes: &mut HashMap<u32, Quete>,
                  dialogues: &mut Vec<Dialogue>) -> bool {
        for quest_id in self.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) {
                if quest.target_id() == item_id {

                    // Augmenter le nombre d'objets pour la quête
                    quest.increment_count();
                    if quest.is_complete() { // Vérifiez si le nombre d'éléments requis a été atteint

                        // Mettre à jour le dialogue correspondant pour refléter l'état d'achèvement de la quête
                        Self::update_dialogues(quest.dialogue_rendu_id, dialogues);

                        // Notifier le joueur que la quête est terminée
                        println!("✅ Quête: {} est complete.", quest.name());
                        println!("Retournez voir le donneur de quête pour récupérer votre prix");
                        return true;
                    }
                }
            }
        }
        false // Si quête n'est pas complete
    }

    /// Lors de tuer un ennemi, cette fonction est appelée et vérifie si l'ennemi
    /// est nécessaire pour terminer une quête
    /// Renvoie true ou false
    fn track_enemy(&mut self,
                   ennemi_id: u32,
                   quetes: &mut HashMap<u32, Quete>,
                   dialogues: &mut Vec<Dialogue>) {

        for quest_id in self.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) { // Accéder à la quête en utilisant son ID
                if quest.target_id() == ennemi_id {

                    // Augmente le nombre d'ennemis requis pour la quête
                    quest.increment_count();
                    if quest.is_complete() { // Check if enough enemies have been defeated

                        // Mettre à jour le dialogue pour l'état d'achèvement de la quête
                        Self::update_dialogues(quest.dialogue_rendu_id, dialogues);

                        // Notifier le joueur que la quête est terminée
                        println!("✅ Quête: {} est complete.", quest.name());
                        println!("Retournez voir le donneur de quête pour récupérer votre prix");

                    }
                }
            }
        }
    }

    fn track_visit(&mut self,
                   room_id: u32,
                   quetes: &mut HashMap<u32, Quete>,
                   dialogues: &mut Vec<Dialogue>) {
        for quest_id in self.quests() {
            if let Some(quest) = quetes.get_mut(&quest_id) { // Accéder à la quête en utilisant son ID
                if quest.target_id() == room_id {
                    if !quest.is_complete() {
                        quest.increment_count();

                        // Mettre à jour le dialogue pour l'état d'achèvement de la quête
                        Self::update_dialogues(quest.dialogue_rendu_id, dialogues);

                        // Notifier le joueur que la quête est terminée
                        println!("✅ Quête: {} est complete.", quest.name());
                        println!("Voyez la bonne personne pour récupérer votre prix");

                    }
                }
            }
        }
    }



}