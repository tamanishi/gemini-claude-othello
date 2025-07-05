use crate::game::Game;
use crate::board::Disc;

#[derive(Debug, Clone, Copy)]
pub enum CpuLevel {
    Easy,
    Medium,
    Hard,
}

pub enum PlayerType {
    Human,
    Cpu(CpuLevel),
}

pub struct Player {
    player_type: PlayerType,
    disc: Disc,
}

impl Player {
    pub fn new(player_type: PlayerType, disc: Disc) -> Self {
        Player { player_type, disc }
    }

    pub fn player_type(&self) -> &PlayerType {
        &self.player_type
    }

    pub fn get_move(&self, game: &Game) -> (usize, usize) {
        match self.player_type {
            PlayerType::Human => panic!("Human move should be handled in main loop"),
            PlayerType::Cpu(level) => self.get_cpu_move(game, level),
        }
    }

    fn get_cpu_move(&self, game: &Game, level: CpuLevel) -> (usize, usize) {
        crate::cpu::get_best_move(game, self.disc, level)
    }

    
}
