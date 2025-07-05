use crate::board::{Board, Disc};

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub current_turn: Disc,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_turn: Disc::Black,
        }
    }

    pub fn current_turn(&self) -> Disc {
        self.current_turn
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn is_valid_move(&self, row: usize, col: usize, player: Disc) -> bool {
        if self.board.get_disc(row, col) != Some(Disc::Empty) {
            return false;
        }

        let opponent = if player == Disc::Black { Disc::White } else { Disc::Black };
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        for (dr, dc) in directions.iter() {
            let mut r = row as i32 + dr;
            let mut c = col as i32 + dc;
            let mut has_opponent_disc = false;

            while r >= 0 && r < 8 && c >= 0 && c < 8 {
                match self.board.get_disc(r as usize, c as usize) {
                    Some(d) if d == opponent => has_opponent_disc = true,
                    Some(d) if d == player => {
                        if has_opponent_disc {
                            return true;
                        }
                        break;
                    }
                    _ => break,
                }
                r += dr;
                c += dc;
            }
        }
        false
    }

    pub fn get_valid_moves(&self, player: Disc) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();
        for r in 0..8 {
            for c in 0..8 {
                if self.is_valid_move(r, c, player) {
                    valid_moves.push((r, c));
                }
            }
        }
        valid_moves
    }

    pub fn make_move(&mut self, row: usize, col: usize, player: Disc) -> bool {
        if !self.is_valid_move(row, col, player) {
            return false;
        }

        self.board.put_disc(row, col, player);
        self.flip_discs(row, col, player);
        self.current_turn = if self.current_turn == Disc::Black { Disc::White } else { Disc::Black };
        true
    }

    fn flip_discs(&mut self, row: usize, col: usize, player: Disc) {
        let opponent = if player == Disc::Black { Disc::White } else { Disc::Black };
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        for (dr, dc) in directions.iter() {
            let mut r = row as i32 + dr;
            let mut c = col as i32 + dc;
            let mut discs_to_flip = Vec::new();

            while r >= 0 && r < 8 && c >= 0 && c < 8 {
                match self.board.get_disc(r as usize, c as usize) {
                    Some(d) if d == opponent => discs_to_flip.push((r as usize, c as usize)),
                    Some(d) if d == player => {
                        for (fr, fc) in &discs_to_flip {
                            self.board.put_disc(*fr, *fc, player);
                        }
                        break;
                    }
                    _ => break,
                }
                r += dr;
                c += dc;
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.get_valid_moves(Disc::Black).is_empty() && self.get_valid_moves(Disc::White).is_empty()
    }

    pub fn count_discs(&self) -> (u32, u32) {
        let mut black_count = 0;
        let mut white_count = 0;
        for r in 0..8 {
            for c in 0..8 {
                match self.board.get_disc(r, c) {
                    Some(Disc::Black) => black_count += 1,
                    Some(Disc::White) => white_count += 1,
                    _ => (),
                }
            }
        }
        (black_count, white_count)
    }
}
