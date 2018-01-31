use std::fmt;

const SIZE: u8 = 11;
const ROWS: u8 = 6;
const COLS: u8 = 7;

pub struct Grid ([u8; SIZE as usize]);
pub enum Disc { R, B }

impl Grid {
    pub fn new() -> Self {
        Grid([0; SIZE as usize])
    }

    pub fn drop(&mut self, col: u8, disc: Disc) {
        for row in 0..ROWS {
            let (index, bit) = Self::index(row, col);
            if let None = self.get(index, bit) {
                self.set(index, bit, disc);
                return
            }
        }
    }

    fn index(row: u8, col: u8) -> (usize, u8) {
        let index = row * COLS + col;
        ((index / 4) as usize, index % 4)
    }

    fn set(&mut self, index: usize, bit: u8, disc: Disc) {
        let &mut Grid(mut grid) = self;
        grid[index] = match (bit, disc) {
            (0, Disc::R) => 0b00000001,
            (0, Disc::B) => 0b00000010,
            (1, Disc::R) => 0b00000100,
            (1, Disc::B) => 0b00001000,
            (2, Disc::R) => 0b00010000,
            (2, Disc::B) => 0b00100000,
            (3, Disc::R) => 0b01000000,
            (3, Disc::B) => 0b10000000,
            _ => unreachable!(),
        };
        println!("Post-set: {:b}", grid[index]);
    }

    fn get(&self, index: usize, bit: u8) -> Option<Disc> {
        let &Grid(grid) = self;
        println!("Pre-mask with index {} and bit {}: {:b}", index, bit, grid[index]);
        let masked = grid[index] & (3 << (bit * 2));
        println!("Post-mask: {:b}", grid[index]);
        match masked >> (bit * 2) {
            0b00000000 => None,
            0b00000001 => Some(Disc::R),
            0b00000010 => Some(Disc::B),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|")?;
        for _ in 0..COLS { write!(f, "-------")?; }
        for row in 0..ROWS {

            write!(f, "\n|")?;
            for _ in 0..COLS { write!(f, "      |")?; }

            write!(f, "\n|")?;
            for col in 0..COLS {
                let (index, bit) = Self::index(row, col);
                match self.get(index, bit) {
                    None => write!(f, "      |")?,
                    Some(Disc::R) => write!(f, "   R   |")?,
                    Some(Disc::B) => write!(f, "   B   |")?,
                }
            }

            write!(f, "\n|")?; 
            for _ in 0..COLS { write!(f, "      |")?; }

            write!(f, "\n-")?;
            for _ in 0..COLS { write!(f, "-------")?; }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use ::grid::*;

    #[test]
    fn test_empty() {
        // println!("{}", Grid::new());
    }

    #[test]
    fn test_single() {
        let mut grid = Grid::new();
        grid.drop(0, Disc::R);
        println!("{}", grid);
    }

}
