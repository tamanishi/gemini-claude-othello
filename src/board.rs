use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disc {
    Black,
    White,
    Empty,
}

impl fmt::Display for Disc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Disc::Black => write!(f, "●"),
            Disc::White => write!(f, "●"),
            Disc::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    grid: [[Disc; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut grid = [[Disc::Empty; 8]; 8];
        grid[3][3] = Disc::White;
        grid[3][4] = Disc::Black;
        grid[4][3] = Disc::Black;
        grid[4][4] = Disc::White;
        Board { grid }
    }

    

    pub fn get_disc(&self, row: usize, col: usize) -> Option<Disc> {
        if row < 8 && col < 8 {
            Some(self.grid[row][col])
        } else {
            None
        }
    }

    pub fn put_disc(&mut self, row: usize, col: usize, disc: Disc) {
        if row < 8 && col < 8 {
            self.grid[row][col] = disc;
        }
    }
}
