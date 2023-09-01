use std::fmt;

#[derive(Debug, Clone)]
pub struct Cell {
    x: i32,
    y: i32,
    pub alive: bool,
    pub next_generation: bool,
}

/// Implementation of a cell for Conway's Game of Life.
///
/// 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
/// 2. Any live cell with two or three live neighbours lives on to the next generation.
/// 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
/// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
impl Cell {
    pub fn new(x: i32, y: i32, alive: bool) -> Cell {
        Cell {
            x,
            y,
            alive,
            next_generation: false,
        }
    }

    /// Get the position of the adjacent cells for the current cell.
    fn get_adjacent_cells_position(&self) -> Vec<[i32; 2]> {
        let adj = vec![
            // top left
            [self.x - 1, self.y - 1],
            // top
            [self.x - 1, self.y],
            // top right
            [self.x - 1, self.y + 1],
            // left
            [self.x, self.y - 1],
            // right
            [self.x, self.y + 1],
            // bottom left
            [self.x + 1, self.y - 1],
            // bottom
            [self.x + 1, self.y],
            // bottom right
            [self.x + 1, self.y + 1],
        ];

        adj
    }

    /// Loops through the cells and returns the cell at the given position.
    fn get_cell_at_position<'a>(
        &self,
        cells: &'a Vec<Vec<Cell>>,
        x: i32,
        y: i32,
    ) -> Option<&'a Cell> {
        for row in cells {
            for cell in row {
                if cell.x == x && cell.y == y {
                    return Some(cell);
                }
            }
        }

        None
    }

    /// Get the number of neighbors for the current cell that are alive or dead depending on the `alive` parameter.
    fn get_neighbors_count<'a>(&self, cells: &'a Vec<Vec<Cell>>, alive: bool) -> Vec<&'a Cell> {
        let mut alive_neighbors = vec![];

        let adjacent_cells_position = self.get_adjacent_cells_position();

        for position in adjacent_cells_position {
            let x = position[0];
            let y = position[1];

            if x < 0 || y < 0 {
                continue;
            }

            let cell = match self.get_cell_at_position(cells, position[0], position[1]) {
                Some(cell) => cell,
                None => continue,
            };

            if cell.alive == alive {
                alive_neighbors.push(cell);
            }
        }

        alive_neighbors
    }

    /// Updates the `next_generation` property of the current cell depending on the number of neighbors that are alive.
    fn should_live(&mut self, cells: &Vec<Vec<Cell>>) {
        if !self.alive {
            return;
        }

        let alive_neighbors = self.get_neighbors_count(cells, true);
        let alive_neighbors_count = alive_neighbors.len();

        self.next_generation = alive_neighbors_count == 2 || alive_neighbors_count == 3
    }

    /// Updates the `next_generation` property of the current cell depending on the number of neighbors that are alive.
    fn should_spawn(&mut self, cells: &Vec<Vec<Cell>>) {
        if self.alive {
            return;
        }

        let alive_neighbors = self.get_neighbors_count(cells, true);
        let alive_neighbors_count = alive_neighbors.len();

        self.next_generation = alive_neighbors_count == 3;
    }

    /// Updates the `alive` property of the current cell to the value of the `next_alive` property.
    /// This should be called after all cells have been updated with the new next_alive value.
    pub fn finalize_generation(&mut self) {
        self.alive = self.next_generation;
    }

    /// Updates the `next_alive` property of the current cell for the current epoch
    pub fn update(&mut self, cells: &Vec<Vec<Cell>>) {
        match self.alive {
            true => self.should_live(cells),
            false => self.should_spawn(cells),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.alive {
                true => "o",
                false => ".",
            }
        )
    }
}
