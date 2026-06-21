use crate::maze::cell::Cell;

struct Grid {
    cells: Vec<Cell>,
    width: u8,
    height: u8,
}

impl Grid {
    pub fn new(width: u8, height: u8) -> Self {
        let mut cells = Vec::new();
        for i in 0..width * height {
            cells.push(Cell::new());
        }
        Self {
            cells,
            width,
            height,
        }
    }
}
