# Prompt de reprise — Projet Maze Generator Rust

Colle ce texte au début d'une nouvelle session Claude pour reprendre le contexte complet.

---

## Contexte à coller

Je travaille sur un projet personnel en Rust : un générateur de labyrinthes avec interface graphique. Je veux me perfectionner en Rust, donc j'ai besoin d'un minimum de challenge mais aussi de pouvoir trouver de l'aide si nécessaire.

**Objectif :** application Rust avec GUI (egui/eframe) permettant de générer des labyrinthes via plusieurs algorithmes, avec visualisation pas à pas de la génération.

---

### Structure du projet

```
maze-gen/
├── src/
│   ├── main.rs           # Racine : mod maze; mod app;
│   ├── app.rs            # struct App, impl eframe::App
│   ├── maze/
│   │   ├── mod.rs        # pub mod cell/wall/grid/display + pub use
│   │   ├── cell.rs       # struct Cell { walls: [bool;4], visited: bool }
│   │   ├── wall.rs       # enum Wall { North=0, South=1, East=2, West=3 }
│   │   ├── grid.rs       # struct Grid { cells: Vec<Cell>, width, height }
│   │   └── display.rs    # rendu egui
│   └── generators/
│       ├── mod.rs        # trait MazeGenerator { fn step(&mut self) -> bool; fn grid(&self) -> &Grid; }
│       ├── dfs.rs
│       ├── prims.rs
│       ├── kruskals.rs
│       └── wilsons.rs
└── Cargo.toml
```

---

### Décisions de conception déjà prises

- **`Wall` est un enum** (North/South/East/West) qui sert d'index sémantique pour `[bool; 4]` dans `Cell`. On évite les magic numbers avec `cell.has_wall(Wall::North)`.
- **`mod.rs` est une façade** : il déclare les sous-modules et réexporte avec `pub use` pour simplifier les imports (`use crate::maze::Cell` plutôt que `use crate::maze::cell::Cell`).
- **Le trait `MazeGenerator`** expose `step(&mut self) -> bool` — l'UI appelle cette méthode à chaque frame. Quand elle retourne `false`, la génération est terminée. L'UI stocke `Box<dyn MazeGenerator>`.
- **Règle modules Rust :** un fichier `.rs` n'existe pour le compilateur que s'il est déclaré avec `mod` dans l'arbre partant de `main.rs`. Les imports utilisent `crate::` comme racine.

---

### Crates utilisées

```toml
[dependencies]
eframe = "0.27"
rand = "0.8"
strum = { version = "0.26", features = ["derive"] }
```

Optionnelles plus tard : `serde` + `serde_json` (sauvegarde), `rayon` (parallélisme).

---

### Algorithmes prévus (par difficulté)

1. **DFS / Recursive Backtracker** — `stack: Vec<(usize, usize)>` (point de départ)
2. **Prim's randomisé** — `frontiers: Vec<(usize, usize, Wall)>`
3. **Kruskal's randomisé** — nécessite Union-Find (Disjoint Set Union)
4. **Wilson's Algorithm** — loop-erased random walk (le plus avancé)

---

### Paradigmes Rust à approfondir dans ce projet

- Trait objects (`Box<dyn Trait>`) vs generics (`impl Trait`)
- Ownership & borrow checker (la grille est possédée par le générateur, empruntée en lecture par l'UI)
- Enums comme state machines (`AppState { Idle, Running, Paused, Done }`)
- Iterators idiomatiques (voisins d'une cellule, filtrage)

---

### Où j'en suis

- Structure de fichiers mise en place
- `Cell`, `Wall`, `Grid` en cours d'implémentation
- Quelques points clarifiés :
  - Pourquoi `Wall` enum + `[bool; 4]` (index sémantique)
  - Pourquoi `mod.rs` est une façade (pas de logique dedans)
  - Comment déclarer les modules dans `main.rs` pour que rust-analyzer fonctionne
  - Syntaxe correcte : `let cells: Vec<Cell> = Vec::new();` (pas `Vec<Cell>::new()`)
  - Imports : `use crate::maze::Cell;` (pas `use cell;`)

---

### Prochaine étape

Finir `Grid` avec ses méthodes (`index`, `cell`, `cell_mut`, `remove_wall_between`), puis implémenter le DFS complet sans animation d'abord pour valider la logique, avant de brancher egui.

---

### Ressources de référence

- [The Rust Book](https://doc.rust-lang.org/book/)
- [egui demo](https://www.egui.rs/#demo) / [egui examples](https://github.com/emilk/egui/tree/master/examples)
- [Blog Jamis Buck — algos labyrinthes](https://weblog.jamisbuck.org/2011/2/7/maze-generation-algorithm-recap)
- [CP-Algorithms — Union-Find](https://cp-algorithms.com/data_structures/disjoint_set_union.html)
