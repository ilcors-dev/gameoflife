use crate::cell::Cell;

pub struct Game {
    generation: u32,
    grid: Vec<Vec<Cell>>,
}

impl Game {
    pub fn new(width: u8, height: u8, alive_starting_cells: Vec<[u8; 2]>) -> Self {
        let mut grid: Vec<Vec<Cell>> = Vec::new();

        for y in 0..height {
            // iterate over rows (y)
            let mut row: Vec<Cell> = Vec::new();

            for x in 0..width {
                // iterate over columns (x)
                let alive = alive_starting_cells
                    .iter()
                    .any(|&target| target[0] == x && target[1] == y);

                row.push(Cell::new(x as i32, y as i32, alive));
            }

            grid.push(row);
        }

        Self {
            generation: 0,
            grid,
        }
    }

    /// Updates the game for the next epoch (generation).
    pub fn update(&mut self) {
        // prepare every cell for the next epoch
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                let others = self.grid.clone();
                let cell = &mut self.grid[i][j];

                cell.update(&others);
            }
        }

        // .. then finalize the next epoch
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                let cell = &mut self.grid[i][j];

                cell.finalize_generation();
            }
        }

        self.generation += 1;
    }

    pub fn display(&self) {
        // clear terminal command
        print!("\x1B[2J\x1B[1;1H");

        let mut output: String = String::new();

        // reverse print to make it look like a grid from bottom left
        for row in self.grid.iter().rev() {
            for cell in row {
                output.push_str(&format!("{} ", cell));
            }

            output.push_str("\n");
        }

        print!("{}", output);
        println!("\nGeneration: {}", self.generation);
    }
}
