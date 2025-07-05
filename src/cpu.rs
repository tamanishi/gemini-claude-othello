use crate::game::Game;
use crate::board::Disc;
use crate::player::CpuLevel;
use rand::Rng;

pub fn get_best_move(game: &Game, player: Disc, level: CpuLevel) -> (usize, usize) {
    let valid_moves = game.get_valid_moves(player);
    if valid_moves.is_empty() {
        return (0, 0); // Should not happen if called correctly
    }

    match level {
        CpuLevel::Easy => get_random_move(&valid_moves),
        CpuLevel::Medium => get_greedy_move(game, &valid_moves, player),
        CpuLevel::Hard => get_minimax_move(game, &valid_moves, player),
    }
}

fn get_random_move(valid_moves: &[(usize, usize)]) -> (usize, usize) {
    let mut rng = rand::rng();
    let index = rng.random_range(0..valid_moves.len());
    valid_moves[index]
}

fn get_greedy_move(game: &Game, valid_moves: &[(usize, usize)], player: Disc) -> (usize, usize) {
    let mut best_move = valid_moves[0];
    let mut max_flipped = 0;

    for &(r, c) in valid_moves {
        let mut temp_game = game.clone();
        temp_game.make_move(r, c, player);
        let (black_count, white_count) = temp_game.count_discs();
        let flipped_count = if player == Disc::Black {
            black_count
        } else {
            white_count
        };

        if flipped_count > max_flipped {
            max_flipped = flipped_count;
            best_move = (r, c);
        }
    }

    best_move
}

fn get_minimax_move(game: &Game, valid_moves: &[(usize, usize)], player: Disc) -> (usize, usize) {
    let mut best_move = valid_moves[0];
    let mut best_score = i32::MIN;

    for &(r, c) in valid_moves {
        let mut temp_game = game.clone();
        temp_game.make_move(r, c, player);
        let score = minimax(&temp_game, 4, false, player, i32::MIN, i32::MAX);
        
        if score > best_score {
            best_score = score;
            best_move = (r, c);
        }
    }

    best_move
}

fn minimax(game: &Game, depth: i32, is_maximizing: bool, player: Disc, alpha: i32, beta: i32) -> i32 {
    if depth == 0 || game.is_game_over() {
        return evaluate_board(game, player);
    }

    let current_player = if is_maximizing { player } else { get_opponent(player) };
    let valid_moves = game.get_valid_moves(current_player);

    if valid_moves.is_empty() {
        return minimax(game, depth - 1, !is_maximizing, player, alpha, beta);
    }

    let mut alpha = alpha;
    let mut beta = beta;

    if is_maximizing {
        let mut max_score = i32::MIN;
        for &(r, c) in &valid_moves {
            let mut temp_game = game.clone();
            temp_game.make_move(r, c, current_player);
            let score = minimax(&temp_game, depth - 1, false, player, alpha, beta);
            max_score = max_score.max(score);
            alpha = alpha.max(score);
            if beta <= alpha {
                break;
            }
        }
        max_score
    } else {
        let mut min_score = i32::MAX;
        for &(r, c) in &valid_moves {
            let mut temp_game = game.clone();
            temp_game.make_move(r, c, current_player);
            let score = minimax(&temp_game, depth - 1, true, player, alpha, beta);
            min_score = min_score.min(score);
            beta = beta.min(score);
            if beta <= alpha {
                break;
            }
        }
        min_score
    }
}

fn evaluate_board(game: &Game, player: Disc) -> i32 {
    let (black_count, white_count) = game.count_discs();
    let my_count = if player == Disc::Black { black_count } else { white_count };
    let opponent_count = if player == Disc::Black { white_count } else { black_count };
    
    let disc_diff = my_count as i32 - opponent_count as i32;
    let mobility = game.get_valid_moves(player).len() as i32 - 
                   game.get_valid_moves(get_opponent(player)).len() as i32;
    
    let positional_score = calculate_positional_score(game, player);
    
    disc_diff + mobility * 5 + positional_score
}

fn calculate_positional_score(game: &Game, player: Disc) -> i32 {
    let position_values = [
        [100, -20, 10, 5, 5, 10, -20, 100],
        [-20, -50, -2, -2, -2, -2, -50, -20],
        [10, -2, -1, -1, -1, -1, -2, 10],
        [5, -2, -1, -1, -1, -1, -2, 5],
        [5, -2, -1, -1, -1, -1, -2, 5],
        [10, -2, -1, -1, -1, -1, -2, 10],
        [-20, -50, -2, -2, -2, -2, -50, -20],
        [100, -20, 10, 5, 5, 10, -20, 100],
    ];

    let mut score = 0;
    for r in 0..8 {
        for c in 0..8 {
            if let Some(disc) = game.board().get_disc(r, c) {
                if disc == player {
                    score += position_values[r][c];
                } else if disc == get_opponent(player) {
                    score -= position_values[r][c];
                }
            }
        }
    }
    score
}

fn get_opponent(player: Disc) -> Disc {
    if player == Disc::Black { Disc::White } else { Disc::Black }
}


