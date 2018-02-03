use std::fmt;

pub const ROWS: u8 = 6;
pub const COLS: u8 = 7;

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

#[derive(Clone, Eq, PartialEq)]
pub struct Grid ([u16; ROWS as usize]);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color { W, B }

impl Grid {
    pub fn new() -> Self { Grid([0; ROWS as usize]) }

    pub fn next(&self, col: u8, color: Color) -> Option<Grid> {
        self.get(ROWS - 1, col).map(|_| {
            let mut grid = self.clone();
            grid.drop(col, color);
            grid
        })
    }

    pub fn drop(&mut self, col: u8, color: Color) {
        for row in 0..ROWS {
            if let None = self.get(row, col) {
                self.set(row, col, color);
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

    pub fn get(&self, row: u8, col: u8) -> Option<Color> {
        let &Grid(grid) = self;
        let row = row as usize;
        match (grid[row] >> col*2) & 0b11 {
            0b00 => None,
            0b01 => Some(Color::W),
            0b10 => Some(Color::B),
            _ => unreachable!(),
        }
    }

    fn count(pieces: u16) -> (i32, i32) {
        let w = (pieces & 0b01010101).count_ones() as i32;
        let b = (pieces & 0b10101010).count_ones() as i32;
        if w > 0 && b > 0 { (0, 0) } else { (w, b) }
    }

    fn set(&mut self, row: u8, col: u8, color: Color) {
        let &mut Grid(ref mut grid) = self;
        let row = row as usize;
        grid[row] |= match (col*2, color) {
            (0  , Color::W) => 0b0000000000000001,
            (0  , Color::B) => 0b0000000000000010,
            (2  , Color::W) => 0b0000000000000100,
            (2  , Color::B) => 0b0000000000001000,
            (4  , Color::W) => 0b0000000000010000,
            (4  , Color::B) => 0b0000000000100000,
            (6  , Color::W) => 0b0000000001000000,
            (6  , Color::B) => 0b0000000010000000,
            (8  , Color::W) => 0b0000000100000000,
            (8  , Color::B) => 0b0000001000000000,
            (10 , Color::W) => 0b0000010000000000,
            (10 , Color::B) => 0b0000100000000000,
            (12 , Color::W) => 0b0001000000000000,
            (12 , Color::B) => 0b0010000000000000,
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
                    Some(Color::W) => write!(f, "   W   |")?,
                    Some(Color::B) => write!(f, "   B   |")?,
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
        grid.drop(0, Color::R);
        println!("{}", grid);
    }

    #[test]
    fn test_column() {
        let mut grid = Grid::new();
        for _ in 0..ROWS {
            grid.drop(3, Color::Y);
        }
        println!("{}", grid);
    }

    #[test]
    fn test_checker() {
        let mut grid = Grid::new();
        for col in 0..COLS {
            for row in 0..ROWS {
                if (col + row) % 2 == 0 {
                    grid.drop(col, Color::R);
                } else {
                    grid.drop(col, Color::Y);
                }
            }
        }
        println!("{}", grid);
    }
}
