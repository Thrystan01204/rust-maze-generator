# Maze Generator — Référence Projet Rust

## Objectif

Application Rust avec interface graphique permettant de générer des labyrinthes via plusieurs algorithmes, avec visualisation de la génération pas à pas.

---

## Structure des fichiers

```
maze-gen/
├── src/
│   ├── main.rs           # Racine : déclare tous les modules
│   ├── app.rs            # État global de l'application, logique UI
│   ├── maze/
│   │   ├── mod.rs        # Déclare et réexporte les sous-modules
│   │   ├── cell.rs       # struct Cell + impl
│   │   ├── wall.rs       # enum Wall
│   │   ├── grid.rs       # struct Grid + impl
│   │   └── display.rs    # Rendu egui du labyrinthe
│   └── generators/
│       ├── mod.rs        # trait MazeGenerator + réexports
│       ├── dfs.rs        # Recursive Backtracker
│       ├── prims.rs      # Prim's randomisé
│       ├── kruskals.rs   # Kruskal's + Union-Find
│       └── wilsons.rs    # Wilson's (LERW)
└── Cargo.toml
```

---

## Système de modules Rust

**Règle fondamentale :** un fichier `.rs` n'existe pour le compilateur que s'il est déclaré avec `mod` quelque part dans l'arbre qui part de `main.rs`.

### main.rs
```rust
mod maze;
mod app;

fn main() {
    // point d'entrée eframe
}
```

### maze/mod.rs
```rust
pub mod cell;
pub mod wall;
pub mod grid;
pub mod display;

// Réexports façade pour simplifier les imports ailleurs
pub use cell::Cell;
pub use wall::Wall;
pub use grid::Grid;
```

### Imports depuis grid.rs (ou n'importe quel fichier)
```rust
use crate::maze::Cell;   // grâce au pub use dans mod.rs
use crate::maze::Wall;
// ou directement :
use crate::maze::cell::Cell;
```

`crate::` désigne toujours la racine (`main.rs`).

---

## Structures de données

### enum Wall (`maze/wall.rs`)
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Wall {
    North = 0,
    South = 1,
    East  = 2,
    West  = 3,
}

impl Wall {
    pub fn opposite(&self) -> Wall {
        match self {
            Wall::North => Wall::South,
            Wall::South => Wall::North,
            Wall::East  => Wall::West,
            Wall::West  => Wall::East,
        }
    }
}
```

### struct Cell (`maze/cell.rs`)
```rust
#[derive(Clone, Debug)]
pub struct Cell {
    pub walls: [bool; 4],   // indexé par Wall as usize
    pub visited: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self { walls: [true; 4], visited: false }
    }

    pub fn has_wall(&self, w: Wall) -> bool {
        self.walls[w as usize]
    }

    pub fn remove_wall(&mut self, w: Wall) {
        self.walls[w as usize] = false;
    }
}
```

**Pourquoi `[bool; 4]` + enum Wall ?**
Le tableau offre la flexibilité de l'indexation et de l'itération. L'enum donne des noms expressifs aux indices, évitant les "magic numbers". Alternative possible : `struct Walls { north: bool, ... }` mais moins pratique pour passer une direction en paramètre.

### struct Grid (`maze/grid.rs`)
```rust
pub struct Grid {
    cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells: Vec<Cell> = vec![Cell::new(); width * height];
        Self { cells, width, height }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[self.index(x, y)]
    }

    pub fn cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let idx = self.index(x, y);
        &mut self.cells[idx]
    }

    pub fn remove_wall_between(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, w: Wall) {
        self.cell_mut(x1, y1).remove_wall(w);
        self.cell_mut(x2, y2).remove_wall(w.opposite());
    }
}
```

---

## Trait MazeGenerator (`generators/mod.rs`)

```rust
pub trait MazeGenerator {
    /// Effectue un pas de génération.
    /// Retourne false quand la génération est terminée.
    fn step(&mut self) -> bool;

    /// Accès en lecture à la grille pour le rendu.
    fn grid(&self) -> &Grid;

    /// Remet le générateur à zéro (nouvelle grille).
    fn reset(&mut self, width: usize, height: usize);
}
```

L'UI stocke `Box<dyn MazeGenerator>` et appelle `step()` à chaque frame. Quand `step()` retourne `false`, l'animation s'arrête.

---

## Algorithmes — par difficulté croissante

### 1. Recursive Backtracker / DFS ⭐
- Choisis une cellule de départ, marque-la visitée
- Tant qu'il reste des voisins non-visités : choisis-en un au hasard, casse le mur, recurse
- Si aucun voisin non-visité : dépile (backtrack)
- **Structure interne :** `stack: Vec<(usize, usize)>`
- **Pattern :** labyrinthe avec de longs couloirs

### 2. Prim's randomisé ⭐⭐
- Commence avec une cellule, ajoute ses murs à un ensemble de "frontières"
- À chaque étape : choisis une frontière au hasard, casse le mur si la cellule de l'autre côté n'est pas visitée
- **Structure interne :** `frontiers: Vec<(usize, usize, Wall)>`
- **Pattern :** labyrinthe plus "ouvert", beaucoup de branches courtes

### 3. Kruskal's randomisé ⭐⭐⭐
- Génère tous les murs intérieurs dans une liste, les mélange
- Pour chaque mur : si les deux cellules sont dans des ensembles disjoints, casse le mur et fusionne les ensembles
- **Structure interne :** Union-Find (Disjoint Set Union)
- **Challenge :** implémenter Union-Find avec union by rank + path compression
- **Pattern :** labyrinthe très uniforme

#### Union-Find (structure pour Kruskal)
```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn find(&mut self, x: usize) -> usize { /* path compression */ }
    fn union(&mut self, x: usize, y: usize) -> bool { /* union by rank, retourne true si fusionné */ }
}
```

### 4. Wilson's Algorithm ⭐⭐⭐⭐
- Commence avec une cellule dans le labyrinthe
- Choisis une cellule hors du labyrinthe, effectue une marche aléatoire jusqu'à atteindre le labyrinthe
- En cas de boucle dans le chemin parcouru : efface la boucle (loop-erased)
- Ajoute le chemin résultant au labyrinthe
- **Propriété :** génère un arbre couvrant uniforme (non biaisé)
- **Challenge :** l'animation est délicate (montrer la marche en cours)

---

## Crates

| Crate | Usage | Lien |
|-------|-------|------|
| `eframe` + `egui` | GUI immediate mode | https://github.com/emilk/egui |
| `rand` | Génération aléatoire, shuffle | https://docs.rs/rand |
| `strum` | Dérive Display/itération sur enum | https://docs.rs/strum |
| `serde` + `serde_json` | Sauvegarde/chargement labyrinthe | https://docs.rs/serde |
| `rayon` | Parallélisme (optionnel) | https://docs.rs/rayon |

### Cargo.toml minimal
```toml
[dependencies]
eframe = "0.27"
rand = "0.8"
strum = { version = "0.26", features = ["derive"] }
```

---

## Paradigmes Rust clés à maîtriser

### Trait objects vs generics
```rust
// Trait object : dispatch dynamique, permet de stocker différents algos
let gen: Box<dyn MazeGenerator> = Box::new(DfsGenerator::new(20, 20));

// Generic : dispatch statique, plus performant mais moins flexible
fn run<G: MazeGenerator>(gen: &mut G) { ... }
```
Utilise `Box<dyn MazeGenerator>` dans l'App pour pouvoir changer d'algo à l'exécution.

### Enums comme state machines
```rust
enum AppState {
    Idle,
    Running { speed: u32 },
    Paused,
    Done,
}
```

### Ownership & borrow checker
Le générateur possède la grille pendant la génération. L'UI emprunte `grid()` en lecture seule pour le rendu. Ces deux opérations ne peuvent pas avoir lieu simultanément — le borrow checker l'enforce.

---

## Interface egui — structure minimale

```rust
// app.rs
pub struct App {
    generator: Box<dyn MazeGenerator>,
    state: AppState,
    speed: u32,  // steps par frame
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if matches!(self.state, AppState::Running { .. }) {
            for _ in 0..self.speed {
                if !self.generator.step() {
                    self.state = AppState::Done;
                    break;
                }
            }
            ctx.request_repaint(); // force la prochaine frame
        }

        egui::SidePanel::left("controls").show(ctx, |ui| {
            // ComboBox pour choisir l'algo
            // Slider pour la vitesse
            // Boutons Start / Pause / Reset
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Dessiner la grille avec ui.painter()
        });
    }
}
```

---

## Ordre de développement suggéré

1. `Wall` + `Cell` + `Grid` avec un rendu statique des murs
2. DFS complet sans animation pour valider la logique
3. Poser le trait `MazeGenerator`, refactoriser DFS
4. Animer step-by-step dans egui (slider de vitesse)
5. Implémenter Prim's
6. Implémenter Kruskal's (avec Union-Find)
7. Implémenter Wilson's
8. Sérialisation (sauvegarder/charger un labyrinthe)

---

## Ressources

- [The Rust Book](https://doc.rust-lang.org/book/) — ch.10 (traits), ch.15 (smart pointers)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Jon Gjengset — Crust of Rust](https://www.youtube.com/c/JonGjengset)
- [egui demo live](https://www.egui.rs/#demo)
- [egui examples GitHub](https://github.com/emilk/egui/tree/master/examples)
- [Mazes for Programmers — Jamis Buck](https://pragprog.com/titles/jbmaze/mazes-for-programmers/)
- [Blog Jamis Buck — récap algorithmes](https://weblog.jamisbuck.org/2011/2/7/maze-generation-algorithm-recap)
- [Red Blob Games](https://www.redblobgames.com/)
- [CP-Algorithms — Union-Find](https://cp-algorithms.com/data_structures/disjoint_set_union.html)
