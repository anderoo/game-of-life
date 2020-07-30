use rand::Rng;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum CellKind {
    ALIVE,
    DEAD,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub kind: CellKind,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.kind == CellKind::ALIVE {
            write!(f, "{}", "◼ ").expect("Unable to write alive cell");
        } else {
            write!(f, "{}", "◻ ").expect("Unable to write dead cell");
        }

        Ok(())
    }
}

impl Cell {
    pub fn new() -> Self {
        if rand::thread_rng().gen_range(0, 5) == 4 {
            Cell { kind: CellKind::ALIVE }
        } else {
            Cell { kind: CellKind::DEAD }
        }
    }

    pub fn next(&mut self, alive_neighbours: i32)  {
        self.kind = match (&self.kind, alive_neighbours) {
            // 1. Any live cell with fewer than two live neighbours
            // dies, as if by underpopulation.
            (CellKind::ALIVE, x) if x < 2 => CellKind::DEAD,

            // 2. Any live cell with two or three live neighbours
            // lives on to the next generation.
            (CellKind::ALIVE, 2) | (CellKind::ALIVE, 3) => CellKind::ALIVE,

            // 3. Any live cell with more than three live neighbours
            // dies, as if by overpopulation.
            (CellKind::ALIVE, x) if x > 3 => CellKind::DEAD,

            // 4. Any dead cell with exactly three live neighbours
            // becomes a live cell, as if by reproduction.
            (CellKind::DEAD, 3) => CellKind::ALIVE,

            // 5. Otherwise, any other cell is marked as dead.
            (_, _) => CellKind::DEAD,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut cell;

        // Case 1
        cell = Cell { kind: CellKind::ALIVE };
        cell.next(0);
        assert_eq!(cell.kind, CellKind::DEAD);

        // Case 2
        cell = Cell { kind: CellKind::ALIVE };
        cell.next(2);
        assert_eq!(cell.kind, CellKind::ALIVE);

        // Case 3
        cell = Cell { kind: CellKind::ALIVE };
        cell.next(4);
        assert_eq!(cell.kind, CellKind::DEAD);

        // Case 4
        cell = Cell { kind: CellKind::DEAD };
        cell.next(3);
        assert_eq!(cell.kind, CellKind::ALIVE);
    }

    #[test]
    fn test_fmt() {
        let mut cell;

        // Alive
        cell = Cell { kind: CellKind::ALIVE };
        assert_eq!(format!("{}", cell), "◼ ");

        // Dead
        cell = Cell { kind: CellKind::DEAD };
        assert_eq!(format!("{}", cell), "◻ ");
    }
}