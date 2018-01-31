use std::fmt;

const ROWS: usize = 6;
const COLS: usize = 7;

pub struct Grid ([u16; ROWS as usize]);
pub enum Disc { R, B }

impl Grid {
    pub fn new() -> Self { Grid([0; ROWS]) }

    pub fn drop(&mut self, col: usize, disc: Disc) {
        for row in 0..ROWS {
            if let None = self.get(row, col) {
                self.set(row, col, disc);
                return
            }
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Disc> {
        let &Grid(grid) = self;
        match (grid[row] & (3 << col * 2)) >> col * 2 {
            0b00000000 => None,
            0b00000001 => Some(Disc::R),
            0b00000010 => Some(Disc::B),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, row: usize, col: usize, disc: Disc) {
        let &mut Grid(ref mut grid) = self;
        grid[row] |= match (col*2, disc) {
            (0  , Disc::R) => 0b0000000000000001,
            (0  , Disc::B) => 0b0000000000000010,
            (2  , Disc::R) => 0b0000000000000100,
            (2  , Disc::B) => 0b0000000000001000,
            (4  , Disc::R) => 0b0000000000010000,
            (4  , Disc::B) => 0b0000000000100000,
            (6  , Disc::R) => 0b0000000001000000,
            (6  , Disc::B) => 0b0000000010000000,
            (8  , Disc::R) => 0b0000000100000000,
            (8  , Disc::B) => 0b0000001000000000,
            (10 , Disc::R) => 0b0000010000000000,
            (10 , Disc::B) => 0b0000100000000000,
            (12 , Disc::R) => 0b0001000000000000,
            (12 , Disc::B) => 0b0010000000000000,
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
                    Some(Disc::B) => write!(f, "   B   |")?,
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
        for row in 0..ROWS {
            grid.drop(3, Disc::B);
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
                    grid.drop(col, Disc::B);
                }
            }
        }
        println!("{}", grid);
    }
}
