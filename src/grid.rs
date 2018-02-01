use std::fmt;

const ROWS: u8 = 6;
const COLS: u8 = 7;

const HORIZONTAL: [(u8, u8); 24] = [
    (0, 0), (0, 1), (0, 2), (0, 3),
    (1, 0), (1, 1), (1, 2), (1, 3),
    (2, 0), (2, 1), (2, 2), (2, 3),
    (3, 0), (3, 1), (3, 2), (3, 3),
    (4, 0), (4, 1), (4, 2), (4, 3),
    (5, 0), (5, 1), (5, 2), (5, 3),
];

const VERTICAL: [(u8, u8); 21] = [
    (0, 0), (1, 0), (2, 0),
    (0, 1), (1, 1), (2, 1),
    (0, 2), (1, 2), (2, 2),
    (0, 3), (1, 3), (2, 3),
    (0, 4), (1, 4), (2, 4),
    (0, 5), (1, 5), (2, 5),
    (0, 6), (1, 6), (2, 6),
];

const LEFT_DIAGONAL: [(u8, u8); 12] = [
    (5, 0), (5, 1), (5, 2), (5, 3),
    (4, 0), (4, 1), (4, 2), (4, 3),
    (3, 0), (3, 1), (3, 2), (3, 3),
];

const RIGHT_DIAGONAL: [(u8, u8); 12] = [
    (0, 0), (0, 1), (0, 2), (0, 3),
    (1, 0), (1, 1), (1, 2), (1, 3),
    (2, 0), (2, 1), (2, 2), (2, 3),
];

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Grid ([u16; ROWS as usize]);

#[derive(Copy, Clone)]
pub enum Disc { R, Y }

impl Grid {
    pub fn new() -> Self { Grid([0; ROWS as usize]) }

    pub fn drop(&mut self, col: u8, disc: Disc) {
        for row in 0..ROWS {
            if let None = self.get(row, col) {
                self.set(row, col, disc);
                return
            }
        }
    }

    pub fn horizontal(&self, row: u8, col: u8) -> (i32, i32) {
        let &Grid(grid) = self;
        let row = row as usize;
        Self::count(grid[row] >> col*2)
    }

    pub fn vertical(&self, row: u8, col: u8) -> (i32, i32) {
        let &Grid(grid) = self;
        let row = row as usize;
        Self::count((grid[row] & (0b11 << col*2))
              + (grid[row + 1] & (0b11 << col*2))
              + (grid[row + 2] & (0b11 << col*2))
              + (grid[row + 3] & (0b11 << col*2)))
    }

    pub fn left_diagonal(&self, row: u8, col: u8) -> (i32, i32) {
        let &Grid(grid) = self;
        let row = row as usize;
        Self::count((grid[row] & (0b11 << col*2))
              + (grid[row - 1] & (0b11 << col*2 + 2))
              + (grid[row - 2] & (0b11 << col*2 + 4))
              + (grid[row - 3] & (0b11 << col*2 + 6)))
    }

    pub fn right_diagonal(&self, row: u8, col: u8) -> (i32, i32) {
        let &Grid(grid) = self;
        let row = row as usize;
        Self::count((grid[row] & (0b11 << col*2))
              + (grid[row + 1] & (0b11 << col*2 + 2))
              + (grid[row + 2] & (0b11 << col*2 + 4))
              + (grid[row + 3] & (0b11 << col*2 + 6)))
    }

    pub fn get(&self, row: u8, col: u8) -> Option<Disc> {
        let &Grid(grid) = self;
        let row = row as usize;
        match (grid[row] >> col*2) & 0b11 {
            0b00 => None,
            0b01 => Some(Disc::R),
            0b10 => Some(Disc::Y),
            _ => unreachable!(),
        }
    }

    fn count(pieces: u16) -> (i32, i32) {
        let r = (pieces & 0b01010101).count_ones() as i32;
        let y = (pieces & 0b10101010).count_ones() as i32;
        if r > 0 && y > 0 { (0, 0) } else { (r, y) }
    }

    fn set(&mut self, row: u8, col: u8, disc: Disc) {
        let &mut Grid(ref mut grid) = self;
        let row = row as usize;
        grid[row] |= match (col*2, disc) {
            (0  , Disc::R) => 0b0000000000000001,
            (0  , Disc::Y) => 0b0000000000000010,
            (2  , Disc::R) => 0b0000000000000100,
            (2  , Disc::Y) => 0b0000000000001000,
            (4  , Disc::R) => 0b0000000000010000,
            (4  , Disc::Y) => 0b0000000000100000,
            (6  , Disc::R) => 0b0000000001000000,
            (6  , Disc::Y) => 0b0000000010000000,
            (8  , Disc::R) => 0b0000000100000000,
            (8  , Disc::Y) => 0b0000001000000000,
            (10 , Disc::R) => 0b0000010000000000,
            (10 , Disc::Y) => 0b0000100000000000,
            (12 , Disc::R) => 0b0001000000000000,
            (12 , Disc::Y) => 0b0010000000000000,
            _ => unreachable!(),
        };
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-")?;
        for _ in 0..COLS { write!(f, "--------")?; }
        for row in 0..ROWS {

            write!(f, "\n|")?;
            for _ in 0..COLS { write!(f, "       |")?; }

            write!(f, "\n|")?;
            for col in 0..COLS {
                match self.get(ROWS - row - 1, col) {
                    None => write!(f, "       |")?,
                    Some(Disc::R) => write!(f, "   R   |")?,
                    Some(Disc::Y) => write!(f, "   Y   |")?,
                }
            }

            write!(f, "\n|")?;
            for _ in 0..COLS { write!(f, "       |")?; }

            write!(f, "\n-")?;
            for _ in 0..COLS { write!(f, "--------")?; }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use ::grid::*;

    #[test]
    fn test_empty() {
        println!("{}", Grid::new());
    }

    #[test]
    fn test_single() {
        let mut grid = Grid::new();
        grid.drop(0, Disc::R);
        println!("{}", grid);
    }

    #[test]
    fn test_column() {
        let mut grid = Grid::new();
        for _ in 0..ROWS {
            grid.drop(3, Disc::Y);
        }
        println!("{}", grid);
    }

    #[test]
    fn test_checker() {
        let mut grid = Grid::new();
        for col in 0..COLS {
            for row in 0..ROWS {
                if (col + row) % 2 == 0 {
                    grid.drop(col, Disc::R);
                } else {
                    grid.drop(col, Disc::Y);
                }
            }
        }
        println!("{}", grid);
    }
}
