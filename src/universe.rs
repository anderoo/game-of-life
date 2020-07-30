use super::cell::{Cell, CellKind};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct Universe {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{}", self.cells[i][j]).expect("Unable to write Cell");
            }

            write!(f, "\n").expect("Unable to write newline");
        }

        Ok(())
    }
}

impl Universe {
    pub fn new(size: usize) -> Self {
        let mut cells: Vec<Vec<Cell>> = vec![vec![]; size];

        for i in 0..size {
            for _ in 0..size {
                cells[i].push(Cell::new());
            }
        }

        Universe { size, cells }
    }

    pub fn tick(&mut self) {
        let prev = self.cells.clone();

        for i in 0..self.size {
            for j in 0..self.size {
                self.cell_tick(&prev, i, j);
            }
        }
    }

    pub fn alive_cells(&self) -> i32 {
        self.count_cell_kinds(&CellKind::ALIVE)
    }

    pub fn dead_cells(&self) -> i32 {
        self.count_cell_kinds(&CellKind::DEAD)
    }

    fn count_cell_kinds(&self, kind: &CellKind) -> i32 {
        self.cells.iter()
            .map(|r| r.iter().filter(|c| c.kind.eq(kind)).count() as i32)
            .sum()
    }

    fn cell_tick(&mut self, prev: &Vec<Vec<Cell>>, i: usize, j: usize) {
        let size = self.size as i32;
        let neighbours: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1), // Top row
            (1, 1), (1, 0), (1, -1),    // Bottom row
            (0, 1), (0, -1),            // Center row
        ];

        let alive_neighbours = neighbours.iter()
            .map(|(x, y)| ((i as i32) + x, (j as i32) + y))
            .filter(|(x, y)| x >= &0 && y >= &0 && x <= &(size - 1) && y <= &(size - 1))
            .filter(|(x, y)| prev[*x as usize][*y as usize].kind == CellKind::ALIVE)
            .count();

        self.cells[i][j].next(alive_neighbours as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_universe() -> Universe {
        Universe {
            size: 3,
            cells: vec![
                vec![dead_cell(), alive_cell(), dead_cell()],
                vec![dead_cell(), dead_cell(), alive_cell()],
                vec![alive_cell(), alive_cell(), alive_cell()],
            ],
        }
    }

    fn alive_cell() -> Cell {
        Cell { kind: CellKind::ALIVE }
    }

    fn dead_cell() -> Cell {
        Cell { kind: CellKind::DEAD }
    }

    #[test]
    fn test_tick() {
        let mut universe = base_universe();

        let expected = Universe {
            size: 3,
            cells: vec![
                vec![dead_cell(), dead_cell(), dead_cell()],
                vec![alive_cell(), dead_cell(), alive_cell()],
                vec![dead_cell(), alive_cell(), alive_cell()],
            ],
        };

        universe.tick();
        assert_eq!(universe, expected);
    }

    #[test]
    fn test_alive_cells() {
        let universe = base_universe();
        assert_eq!(universe.alive_cells(), 5);
    }

    #[test]
    fn test_dead_cells() {
        let universe = base_universe();
        assert_eq!(universe.dead_cells(), 4);
    }

    #[test]
    fn test_fmt() {
        let universe = base_universe();
        let expected = "\
            ◻ ◼ ◻ \n\
            ◻ ◻ ◼ \n\
            ◼ ◼ ◼ \n\
        ";

        assert_eq!(format!("{}", universe), expected);
    }
}