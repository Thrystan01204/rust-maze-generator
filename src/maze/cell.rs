use crate::maze::wall::Wall;

pub struct Cell {
    walls: [bool; 4],
    visited: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            walls: [false; 4],
            visited: false,
        }
    }

    fn is_visited(&self) -> bool {
        self.visited
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn is_wall(&self, direction: Wall) -> bool {
        self.walls[direction as usize]
    }

    fn set_wall(&mut self, direction: Wall, is_wall: bool) {
        self.walls[direction as usize] = is_wall;
    }
}
