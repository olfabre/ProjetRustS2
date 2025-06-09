📌 Objectif : Un RPG textuel simple avec parallélisme
On garde l'idée d'un jeu de rôle textuel inspiré des "livres dont vous êtes le héros", avec :

Un monde dynamique (zones explorables)
Un scénario aléatoire influencé par des jets de dés
Une interaction avec des PNJ et objets
Une gestion du temps qui influence le gameplay
Du parallélisme pour les événements (ex. PNJ évoluant en arrière-plan)
⚡ Ce qu'on simplifie :
✅ Moins de complexité dans le système de combat (on garde un simple jet de dés).
✅ Un seul joueur (pas de gestion multijoueur).
✅ Un nombre limité d'objets, PNJ et zones (3 zones, 3 PNJ, 3 objets interactifs).
✅ Un scénario prédéfini avec des variations au lieu d'un graphe complexe.

# Documentation du Projet Rust S2

## 📚 Documentation Technique

La documentation technique complète est disponible en générant la documentation avec Cargo :
```bash
cargo doc --no-deps
cargo doc --open
```

Vous pouvez accéder directement à la documentation générée à l'adresse :
`./target/doc/projet_rust_s2/index.html`

## 🎮 Structure du Projet

### Models
Le module `models` est le cœur du jeu, contenant tous les modèles et la logique métier :

- `game.rs` : Gestion du jeu
  - Structure `Game` pour la coordination des composantes
  - Gestion de l'état du jeu
  - Boucle principale du jeu

- `dialogue.rs` : Système de dialogue
  - Gestion des conversations avec les PNJ
  - Arbres de dialogue
  - Choix de réponses

- `combat.rs` : Système de combat
  - Règles de combat
  - Calcul des dégâts
  - Gestion des tours de combat

- `worlds.rs` : Gestion des mondes
  - Structure du monde de jeu
  - Gestion des zones
  - Événements du monde

- `tracker.rs` : Suivi des quêtes
  - Progression des quêtes
  - Objectifs et récompenses
  - État des quêtes

### Entities
Le module `entities` contient toutes les entités du jeu :

- `character.rs` : Personnage joueur
  - Statistiques et attributs
  - Inventaire
  - Progression (niveau, expérience)

- `Enemy.rs` : Ennemis
  - Types d'ennemis
  - Comportements de combat
  - Loot

- `pnj.rs` : Personnages non-joueurs
  - Dialogues
  - Quêtes
  - Interactions

- `room.rs` : Salles du jeu
  - Description
  - Objets présents
  - PNJ présents

- `quete.rs` : Système de quêtes
  - Objectifs
  - Récompenses
  - Conditions de complétion

- `item.rs` : Objets du jeu
  - Types d'objets
  - Effets
  - Valeur

- `inventory.rs` : Inventaire
  - Gestion des objets
  - Capacité
  - Organisation

- `vivant.rs` : Entités vivantes
  - Points de vie
  - Statistiques de base
  - État (vivant/mort)

### Traits
Les traits définissent les comportements communs :

- `Combattant` : Système de combat
  - Attaques
  - Défense
  - Calcul des dégâts

- `Descriptible` : Description des entités
  - Textes descriptifs
  - Informations affichées

- `Interactable` : Interactions
  - Actions possibles
  - Réponses aux interactions

- `MoneyManager` : Gestion de l'argent
  - Transactions
  - Solde
  - Vérifications

## 🚀 Installation et Utilisation

1. Cloner le projet :
```bash
git clone [URL_DU_REPO]
cd ProjetRustS2
```

2. Compiler le projet :
```bash
cargo build
```

3. Lancer le jeu :
```bash
cargo run
```

## 💻 Exemples d'Utilisation

### Création d'un personnage
```rust
let mut character = Character::new("Hero", "Un brave aventurier");
character.add_experience(100);
```

### Système de combat
```rust
let mut enemy = Enemy::new("Gobelin", 100, 10);
character.attaquer(&mut enemy);
```

### Gestion des quêtes
```rust
let quete = Quete::new("Chasse au dragon", "Tuer le dragon");
character.ajouter_quete(quete);
```

## 🔧 Développement

### Génération de la documentation
```bash
# Générer la documentation
cargo doc --no-deps

# Ouvrir la documentation dans le navigateur
cargo doc --open
```

### Tests
```bash
# Exécuter tous les tests
cargo test

# Exécuter les tests avec affichage de la sortie
cargo test -- --nocapture
```

## 📝 Licence

Ce projet est sous licence [INSÉRER LICENCE]

## 👥 Auteurs

[INSÉRER NOMS DES AUTEURS]

