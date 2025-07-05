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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_new() {
        let board = Board::new();
        assert_eq!(board.get_disc(3, 3), Some(Disc::White));
        assert_eq!(board.get_disc(3, 4), Some(Disc::Black));
        assert_eq!(board.get_disc(4, 3), Some(Disc::Black));
        assert_eq!(board.get_disc(4, 4), Some(Disc::White));
        assert_eq!(board.get_disc(0, 0), Some(Disc::Empty));
    }

    #[test]
    fn test_get_disc() {
        let board = Board::new();
        assert_eq!(board.get_disc(0, 0), Some(Disc::Empty));
        assert_eq!(board.get_disc(8, 8), None);
        assert_eq!(board.get_disc(7, 7), Some(Disc::Empty));
    }

    #[test]
    fn test_put_disc() {
        let mut board = Board::new();
        board.put_disc(0, 0, Disc::Black);
        assert_eq!(board.get_disc(0, 0), Some(Disc::Black));
        
        board.put_disc(8, 8, Disc::Black);
        assert_eq!(board.get_disc(8, 8), None);
    }

    #[test]
    fn test_disc_display() {
        assert_eq!(format!("{}", Disc::Black), "●");
        assert_eq!(format!("{}", Disc::White), "●");
        assert_eq!(format!("{}", Disc::Empty), ".");
    }
}
