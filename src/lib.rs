mod utils;

use std::fmt;
use std::usize;
use wasm_bindgen::prelude::*;

use js_sys;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }
//
// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}. How are you?", name));
// }
//

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_position(&self, idx: usize) -> (u32, u32) {
        // column can be derived by modulus operation
        let column = idx % (self.width as usize);
        // subtract column from idx and divide by width to get row
        let row = (idx - column) / (self.width as usize);

        (row as u32, column as u32)
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            //Only 3 values: 0, 1 or a wrapparound for the total height.
            //Meaningful when combined with modulus.
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // Handle case where delta is set to central pixel
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr() //as_ptr returns pointer
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive // No ; because it's return?
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width: width,
            height: height,
            cells: cells,
        }
    }

    pub fn blank() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height).map(|_| Cell::Dead).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn random() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    // new glider, rough implementation
    pub fn glider() -> Universe {
        let width = 3;
        let height = 3;

        let cells: Vec<Cell> = [
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
        ]
        .to_vec();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn place(&mut self, element: &Universe, row: u32, column: u32) {
        let mut next = self.cells.clone();

        for element_idx in 0..(element.width * element.height) as usize {
            let donor_cell = element.cells[element_idx];
            let (delta_row, delta_col) = element.get_position(element_idx);

            let idx = self.get_index(row + delta_row - 1, column + delta_col - 1);
            let acceptor_cell = self.cells[idx];

            let next_cell = match (acceptor_cell, donor_cell) {
                (Cell::Alive, Cell::Alive) => Cell::Dead,
                (Cell::Alive, Cell::Dead) => Cell::Alive,
                (Cell::Dead, Cell::Alive) => Cell::Alive,
                (Cell::Dead, Cell::Dead) => Cell::Dead,
            };

            next[idx] = next_cell;
        }

        self.cells = next
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
