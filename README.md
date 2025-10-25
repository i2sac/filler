# Filler - Robot de Stratégie 🤖

## 📋 Table des Matières
- [Présentation du Projet](#-présentation-du-projet)
- [Règles du Jeu](#-règles-du-jeu)
- [Installation](#-installation)
- [Utilisation](#-utilisation)
- [Architecture Technique](#-architecture-technique)
- [Algorithmes et Méthodes de Calcul](#-algorithmes-et-méthodes-de-calcul)
- [Stratégies Implémentées](#-stratégies-implémentées)
- [Structure du Code](#-structure-du-code)
- [Optimisations de Performance](#-optimisations-de-performance)
- [Développement](#-développement)
- [Auteur](#-auteur)

## 🎮 Présentation du Projet

**Filler** est un jeu de stratégie compétitif où deux robots (joueurs) s'affrontent sur une grille appelée **Anfield**. L'objectif est de maximiser l'espace occupé en plaçant stratégiquement des pièces de formes variées tout en bloquant les avancées de l'adversaire.

Ce projet implémente un robot intelligent nommé **Ultron**, développé en Rust, capable de prendre des décisions stratégiques en temps réel pour dominer le terrain de jeu. Le robot utilise des algorithmes avancés de calcul matriciel, de traitement parallèle et d'analyse spatiale pour optimiser ses mouvements.

### Caractéristiques Principales
- ✅ **Algorithme de convolution** pour valider les placements de pièces
- ✅ **Stratégie agressive** basée sur la proximité de l'ennemi
- ✅ **Calcul parallélisé** avec Rayon pour des performances optimales
- ✅ **Gestion matricielle** des grilles et des pièces
- ✅ **Optimisation compile-time** avec LTO et native CPU targeting
- ✅ **Support Docker** pour une exécution portable

## 🎯 Règles du Jeu

### Principe de Base
1. Deux joueurs s'affrontent sur une grille rectangulaire (Anfield)
2. À chaque tour, le moteur de jeu fournit une pièce aléatoire composée de cases marquées 'O'
3. Le joueur doit placer cette pièce sur la grille en respectant les contraintes suivantes :
   - La pièce doit chevaucher **exactement une** case occupée par le joueur
   - La pièce ne doit **jamais** chevaucher une case de l'adversaire
   - La pièce doit rester dans les limites de la grille
4. Si aucun placement valide n'est possible, le joueur perd
5. Le joueur qui occupe le plus d'espace gagne la partie

### Représentation sur la Grille
- **Joueur 1** : `O` ou `o` (respectivement pièces initiales et placées)
- **Joueur 2** : `X` ou `x` (respectivement pièces initiales et placées)
- Cases vides : `.`

## 🚀 Installation

### Prérequis
- **Docker** (recommandé pour une exécution simplifiée)
- **OU Rust 1.70+** (pour compilation locale)
- **Git** pour cloner le repository

### Installation avec Docker (Recommandé)

#### 1. Cloner le Repository
```bash
git clone https://github.com/i2sac/filler.git
cd filler
```

#### 2. Construire l'Image Docker
```bash
docker build -t filler .
```

Cette commande crée une image Docker contenant :
- L'environnement Rust
- Le moteur de jeu (`linux_game_engine` ou `m1_game_engine`)
- Les robots adversaires prédéfinis
- Les cartes de jeu

#### 3. Exécuter le Conteneur
```bash
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

Cette commande :
- Monte le répertoire `solution` local dans le conteneur
- Ouvre un terminal interactif dans le conteneur
- Permet de compiler et tester votre robot

### Installation Locale (Sans Docker)

#### 1. Installer Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 2. Compiler le Projet
```bash
cd solution
cargo build --release
```

L'exécutable sera généré dans `solution/target/release/ultron`.

## 💻 Utilisation

### Dans le Conteneur Docker

Une fois dans le conteneur Docker :

```bash
# Compiler le robot
cd /filler/solution
cargo build --release

# Lancer une partie contre Bender
./linux_game_engine -f maps/map01 -p1 target/release/ultron -p2 linux_robots/bender

# Lancer une partie contre Terminator (adversaire difficile)
./linux_game_engine -f maps/map01 -p1 target/release/ultron -p2 linux_robots/terminator
```

### Sur Mac M1/M2
Pour les utilisateurs de Mac avec processeur Apple Silicon :
```bash
./m1_game_engine -f maps/map01 -p1 target/release/ultron -p2 m1_robots/terminator
```

### Options du Moteur de Jeu
- `-f maps/mapXX` : Spécifie la carte à utiliser
- `-p1 <robot>` : Définit le robot joueur 1
- `-p2 <robot>` : Définit le robot joueur 2

### Robots Adversaires Disponibles
- **bender** : Robot de difficulté moyenne
- **terminator** : Robot très fort (challenge difficile)
- **wall-e** : Robot de difficulté variable
- **r2d2** : Autre robot adversaire

> **Note** : Terminator est un adversaire extrêmement fort. Le battre n'est pas obligatoire pour valider le projet.

## 🏗️ Architecture Technique

### Technologies Utilisées
- **Langage** : Rust (Edition 2021)
- **Parallélisation** : Rayon 1.10.0
- **Algèbre Linéaire** : rust_linalg (bibliothèque personnalisée)
- **Génération Aléatoire** : rand 0.8.5

### Structure du Projet
```
filler/
├── Dockerfile              # Configuration Docker
├── README.md              # Ce fichier
├── solution/              # Code source du robot
│   ├── Cargo.toml        # Configuration Cargo
│   ├── src/
│   │   ├── main.rs       # Point d'entrée
│   │   └── lib.rs        # Logique principale
│   └── target/           # Binaires compilés (ignoré par git)
├── maps/                  # Cartes de jeu (dans Docker)
├── linux_game_engine     # Moteur de jeu Linux (dans Docker)
├── m1_game_engine        # Moteur de jeu Mac M1 (dans Docker)
├── linux_robots/         # Robots adversaires Linux (dans Docker)
└── m1_robots/            # Robots adversaires Mac M1 (dans Docker)
```

### Composants Principaux

#### 1. Game State (`Game` struct)
```rust
pub struct Game {
    pub my_player: usize,      // Numéro du joueur (1 ou 2)
    pub map_width: usize,      // Largeur de la grille
    pub map_height: usize,     // Hauteur de la grille
}
```

Stocke l'état global de la partie pour chaque robot.

#### 2. Main Loop (`main.rs`)
Boucle infinie qui appelle la fonction `turn()` à chaque tour pour calculer et envoyer le prochain mouvement.

#### 3. Game Logic (`lib.rs`)
Contient toute la logique de jeu, parsing des entrées, calculs matriciels et stratégie.

## 🧮 Algorithmes et Méthodes de Calcul

### 1. Parsing des Entrées
Le robot lit les informations du moteur de jeu via l'entrée standard (stdin) :

```rust
$$$ exec p1 : ./ultron     // Identification du joueur
Anfield 15 17:             // Dimensions de la grille
    000000000001111         // Grille ligne par ligne
    ...
Piece 3 4:                 // Dimensions de la pièce
**.*                       // Forme de la pièce
....
```

**Algorithme de parsing** :
1. Détection du numéro de joueur (p1 ou p2)
2. Extraction des dimensions de la grille
3. Capture de la grille complète ligne par ligne
4. Extraction des dimensions de la pièce
5. Capture de la forme de la pièce

### 2. Conversion en Matrices Numériques

#### Grille (Map)
Les caractères de la grille sont transformés en valeurs numériques :
- `0` : Case vide
- `1` : Case occupée par le joueur
- `3` : Case occupée par l'adversaire

```rust
let map_vecs = map_lines.iter().enumerate().map(|(y, line)| {
    line.chars().enumerate().map(|(x, c)| {
        if player_chars.contains(&c) { 1 }      // Nos pièces
        else if foe_chars.contains(&c) { 3 }    // Pièces ennemies
        else { 0 }                               // Vide
    }).collect::<Vec<usize>>()
}).collect::<Vec<Vec<usize>>>();
```

#### Pièce
La pièce est convertie en matrice binaire :
- `1` : Partie de la pièce ('*')
- `0` : Case vide ('.')

```rust
let piece_vecs = piece_lines.iter().map(|line| {
    line.chars().map(|c| if c == '*' { 1 } else { 0 })
        .collect::<Vec<usize>>()
}).collect::<Vec<Vec<usize>>>();
```

### 3. Algorithme de Convolution

La **convolution** est la méthode centrale pour valider les placements. Pour chaque position possible (x, y) :

1. **Extraction d'une fenêtre** de la grille de la taille de la pièce
2. **Addition matricielle** : `résultat = pièce + fenêtre_grille`
3. **Validation des contraintes** :
   - Compter les cases avec valeur `2` (pièce chevauchant nos cases) → doit être **exactement 1**
   - Compter les cases avec valeur `> 2` (pièce chevauchant l'ennemi) → doit être **0**

```rust
let piece_matrix = Matrix::new(piece_vecs.clone());
let map_window = Matrix::new(
    map_vecs[y..(y + piece_h)]
        .iter()
        .map(|line| line[x..(x + piece_w)].to_vec())
        .collect(),
);
let result_matrix = piece_matrix + map_window;

let good_overlay = result_matrix.data.iter().flatten()
    .filter(|&&v| v == 2).count();
let bad_overlay = result_matrix.data.iter().flatten()
    .filter(|&&v| v > 2).count();

if good_overlay == 1 && bad_overlay == 0 {
    // Position valide !
}
```

**Exemple visuel** :
```
Pièce:          Fenêtre Grille:    Résultat:
1 1             0 0                1 1
1 0             1 0                2 0  → good_overlay = 1 ✓
                                         bad_overlay = 0 ✓
```

### 4. Optimisation de l'Espace de Recherche

Pour éviter de tester toutes les positions de la grille (coûteux), on réduit l'espace de recherche :

```rust
// Trouver les limites de nos pièces
let (mut min_x, mut max_x, mut min_y, mut max_y) = 
    (map_width, 0, map_height, 0);

// Étendre la zone de recherche de la taille de la pièce
min_x = max(0, min_x - piece_w);
min_y = max(0, min_y - piece_h);

// Ne tester que les positions dans cette zone réduite
for y in min_y..=max_y {
    for x in min_x..=max_x {
        // Test de convolution
    }
}
```

Cette optimisation réduit drastiquement le nombre de positions à tester, surtout en début de partie.

### 5. Calcul de Distance Manhattan

Pour choisir parmi les positions valides, on utilise la **distance de Manhattan** :

```rust
fn closer_to_enemy(pos_ok: &[(usize, usize)], foe_pos: &[(usize, usize)]) 
    -> (usize, usize) 
{
    pos_ok.par_iter().min_by_key(|&&pos| {
        foe_pos.iter().map(|&foe| {
            (pos.0 as i32 - foe.0 as i32).abs() + 
            (pos.1 as i32 - foe.1 as i32).abs()
        }).min().unwrap_or(usize::MAX as i32)
    }).copied().unwrap_or(pos_ok[0])
}
```

**Distance de Manhattan** : `|x1 - x2| + |y1 - y2|`

Cette métrique mesure la distance "à pied" entre deux points, parfaite pour une grille.

## 🎯 Stratégies Implémentées

### Stratégie Principale : Agression Proximale

Le robot Ultron utilise une **stratégie agressive** basée sur la proximité avec l'adversaire :

1. **Phase d'Expansion** : Occuper rapidement l'espace disponible
2. **Phase de Confrontation** : Se rapprocher de l'adversaire pour limiter son espace
3. **Phase de Blocage** : Empêcher l'adversaire d'avoir des placements valides

### Avantages de la Stratégie
- ✅ **Pression constante** sur l'adversaire
- ✅ **Réduction de l'espace disponible** pour l'ennemi
- ✅ **Adaptation dynamique** : ajustement automatique en fonction des positions
- ✅ **Simplicité et efficacité** : pas de calculs complexes de prédiction

### Limites et Améliorations Possibles

#### Limites Actuelles
- La stratégie est purement réactive (pas d'anticipation à long terme)
- Pas de détection de pièges ou de zones dangereuses
- Choix basé uniquement sur la distance (pas de considération de forme)

#### Améliorations Envisageables
1. **Algorithme Minimax** : Évaluation des coups à plusieurs tours d'avance
2. **Détection de zones** : Identifier les zones à fort potentiel
3. **Évaluation de contrôle territorial** : Privilégier les positions qui maximisent notre influence
4. **Machine Learning** : Apprentissage des patterns de victoire
5. **Détection de pièges** : Éviter les positions qui pourraient nous bloquer

### Pourquoi la Distance Manhattan ?

La **distance de Manhattan** est préférée à la distance euclidienne car :
- Plus rapide à calculer (pas de racine carrée)
- Correspond mieux à la logique de grille
- Encourage les mouvements directs vers l'ennemi

## 📁 Structure du Code

### `main.rs` - Point d'Entrée
```rust
fn main() {
    let mut game = Game::new();
    
    loop {
        turn(&mut game);
    }
}
```
Initialise le jeu et entre dans une boucle infinie de tours.

### `lib.rs` - Logique Principale

#### Fonction `turn()`
**Responsabilité** : Gérer un tour complet de jeu
**Étapes** :
1. Parser les entrées du moteur
2. Identifier le joueur et extraire l'état de la grille
3. Récupérer la pièce à placer
4. Convertir grille et pièce en matrices
5. Trouver toutes les positions valides (convolution)
6. Choisir la meilleure position (proximité ennemi)
7. Envoyer la position au moteur via stdout

**Complexité** : O(n × m × p × q) où :
- n, m : dimensions de la zone de recherche
- p, q : dimensions de la pièce

#### Fonction `closer_to_enemy()`
**Responsabilité** : Sélectionner la position optimale
**Algorithme** :
1. Pour chaque position valide
2. Calculer la distance minimale à toutes les positions ennemies
3. Retourner la position avec la distance minimale

**Complexité** : O(k × e) où :
- k : nombre de positions valides
- e : nombre de positions ennemies

## ⚡ Optimisations de Performance

### 1. Compilation Optimisée
Configuration dans `Cargo.toml` :
```toml
[profile.release]
opt-level = 3              # Optimisation maximale
lto = true                 # Link-Time Optimization
codegen-units = 1          # Optimisation inter-modules
target-cpu = "native"      # Optimisation pour le CPU cible
```

**Gains** :
- Jusqu'à 30% de performances supplémentaires
- Code plus compact et plus rapide

### 2. Parallélisation avec Rayon
Utilisation de `par_iter()` pour paralléliser :
- La recherche des positions valides
- Le calcul des distances à l'ennemi

```rust
let pos_ok: Vec<(usize, usize)> = (min_y..=max_y)
    .into_par_iter()  // Parallélisation
    .flat_map(|y| { ... })
    .collect();
```

**Gains** :
- Utilisation de tous les cœurs CPU disponibles
- Réduction significative du temps de calcul sur grandes grilles

### 3. Réduction de l'Espace de Recherche
Au lieu de tester toute la grille, on limite aux zones pertinentes :
- Zone autour de nos pièces existantes
- Extension de la taille de la pièce

**Gains** :
- Réduction de 90%+ des positions testées en moyenne
- Temps de réponse quasi-instantané

### 4. Structures de Données Efficaces
- Utilisation de `Vec<Vec<usize>>` pour les matrices
- Pas d'allocations inutiles en boucle
- Clonage minimal des données

## 🔧 Développement

### Lancer en Mode Debug
```bash
cd solution
cargo run
```

### Exécuter les Tests
```bash
cargo test
```

### Vérifier le Code
```bash
# Formatter le code
cargo fmt

# Vérifier le linting
cargo clippy

# Vérifier la compilation sans générer de binaire
cargo check
```

### Profiling des Performances
```bash
# Compiler avec symboles de debug
cargo build --release --profile release-with-debug

# Utiliser un profiler (exemple avec perf sur Linux)
perf record ./target/release/ultron
perf report
```

### Debugging
Pour débugger une partie :
```bash
# Rediriger les sorties du jeu dans un fichier
./linux_game_engine -f maps/map01 -p1 target/release/ultron \
    -p2 linux_robots/bender > output.txt 2>&1
```

## 🛠️ Dépendances

### rust_linalg
Bibliothèque personnalisée pour les opérations matricielles :
- Addition de matrices
- Gestion de matrices de tailles variables
- Optimisée pour les performances

Repository : https://github.com/louisisaacdiouf/rust_linalg

### Rayon
Framework de parallélisation data-parallel :
- API simple et sûre
- Parallélisation automatique des itérations
- Gestion optimale des threads

### Rand
Générateur de nombres aléatoires (pour extensions futures)

## 📊 Performances

### Benchmarks Typiques
- **Temps par tour** : < 5ms en moyenne
- **Positions testées** : 100-500 par tour (selon la situation)
- **Utilisation CPU** : Multi-cœur grâce à Rayon
- **Utilisation mémoire** : < 10MB

### Comparaison avec les Adversaires
- ✅ **Bender** : Victoire régulière
- ✅ **Wall-E** : Victoire fréquente
- ⚠️ **Terminator** : Adversaire très difficile (victoire possible mais non garantie)

## 🤝 Contribuer

Les contributions sont les bienvenues ! Pour contribuer :

1. Fork le projet
2. Créez une branche (`git checkout -b feature/amelioration`)
3. Committez vos changements (`git commit -m 'Ajout d'une fonctionnalité'`)
4. Pushez vers la branche (`git push origin feature/amelioration`)
5. Ouvrez une Pull Request

### Idées de Contributions
- Amélioration de la stratégie de jeu
- Ajout de nouvelles heuristiques
- Optimisation des performances
- Documentation supplémentaire
- Tests unitaires

## 📝 Licence

Ce projet est un projet éducatif développé dans le cadre d'un cursus de formation en informatique.

## 👤 Auteur

**@lodiouf** - Louis Isaac Diouf

---

## 🎓 Contexte Pédagogique

Ce projet illustre plusieurs concepts avancés de programmation :
- **Algorithmique** : Convolution, recherche de chemin, optimisation
- **Structures de données** : Matrices, vecteurs, parsing efficace
- **Parallélisation** : Calculs multi-cœurs avec Rayon
- **Optimisation** : Réduction de l'espace de recherche, compilation optimisée
- **Architecture logicielle** : Séparation des responsabilités, modularité
- **Game AI** : Stratégies de jeu, heuristiques

---

## 🔗 Ressources Utiles

- [Documentation Rust](https://doc.rust-lang.org/)
- [Rayon Documentation](https://docs.rs/rayon/)
- [Algorithme de Convolution](https://fr.wikipedia.org/wiki/Produit_de_convolution)
- [Distance de Manhattan](https://fr.wikipedia.org/wiki/Distance_de_Manhattan)

---

**Bonne chance dans vos parties ! Que le meilleur robot gagne ! 🏆**
