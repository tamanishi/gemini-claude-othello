mod board;
mod cpu;
mod game;
mod player;

use board::{Board, Disc};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::Game;
use player::{Player, PlayerType, CpuLevel};
use std::io::{stdout, Stdout};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;

    let game_mode = select_game_mode(&mut stdout)?;

    let mut game = Game::new();
    let player1 = Player::new(PlayerType::Human, Disc::Black);
    let player2 = Player::new(game_mode, Disc::White);
    let mut cursor_pos = (0, 0);

    let result = run_game_loop(&mut stdout, &mut game, &player1, &player2, &mut cursor_pos);

    terminal::disable_raw_mode()?;
    execute!(stdout, Show, LeaveAlternateScreen)?;
    result
}

fn select_game_mode(stdout: &mut Stdout) -> std::io::Result<PlayerType> {
    loop {
        draw_game_mode_selection(stdout, "")?;
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('1') => return Ok(PlayerType::Human),
                KeyCode::Char('2') => return Ok(select_cpu_level(stdout)?),
                KeyCode::Char('q') => {
                    terminal::disable_raw_mode()?;
                    execute!(stdout, Show, LeaveAlternateScreen)?;
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }
}

fn select_cpu_level(stdout: &mut Stdout) -> std::io::Result<PlayerType> {
    loop {
        draw_cpu_level_selection(stdout, "")?;
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('1') => return Ok(PlayerType::Cpu(CpuLevel::Easy)),
                KeyCode::Char('2') => return Ok(PlayerType::Cpu(CpuLevel::Medium)),
                KeyCode::Char('3') => return Ok(PlayerType::Cpu(CpuLevel::Hard)),
                KeyCode::Char('b') => return select_game_mode(stdout),
                KeyCode::Char('q') => {
                    terminal::disable_raw_mode()?;
                    execute!(stdout, Show, LeaveAlternateScreen)?;
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }
}

fn run_game_loop(
    stdout: &mut Stdout,
    game: &mut Game,
    player1: &Player,
    player2: &Player,
    cursor_pos: &mut (u16, u16),
) -> std::io::Result<()> {
    loop {
        draw_board(stdout, game.board(), *cursor_pos)?;
        draw_info(stdout, game)?;

        let current_player_disc = game.current_turn();
        let current_player = if current_player_disc == Disc::Black {
            player1
        } else {
            player2
        };

        if game.is_game_over() {
            draw_game_over(stdout, game)?;
            break;
        }

        let valid_moves = game.get_valid_moves(current_player_disc);
        if valid_moves.is_empty() {
            game.current_turn = if current_player_disc == Disc::Black {
                Disc::White
            } else {
                Disc::Black
            };
            continue;
        }

        let (row, col) = match current_player.player_type() {
            PlayerType::Human => {
                let human_move = get_human_input(stdout, game, cursor_pos, current_player_disc)?;
                if human_move.is_none() {
                    // 'q' was pressed
                    return Ok(());
                }
                human_move.unwrap()
            }
            PlayerType::Cpu(level) => {
                show_cpu_thinking(stdout, game, *level)?;
                player2.get_move(game)
            }
        };

        game.make_move(row, col, current_player_disc);
    }
    Ok(())
}

fn get_human_input(
    stdout: &mut Stdout,
    game: &Game,
    cursor_pos: &mut (u16, u16),
    player_disc: Disc,
) -> std::io::Result<Option<(usize, usize)>> {
    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => cursor_pos.0 = cursor_pos.0.saturating_sub(1),
                KeyCode::Down => cursor_pos.0 = (cursor_pos.0 + 1).min(7),
                KeyCode::Left => cursor_pos.1 = cursor_pos.1.saturating_sub(1),
                KeyCode::Right => cursor_pos.1 = (cursor_pos.1 + 1).min(7),
                KeyCode::Enter | KeyCode::Char(' ') => {
                    if game.is_valid_move(cursor_pos.0 as usize, cursor_pos.1 as usize, player_disc)
                    {
                        return Ok(Some((cursor_pos.0 as usize, cursor_pos.1 as usize)));
                    }
                }
                KeyCode::Char('q') => return Ok(None),
                _ => {}
            }
            draw_board(stdout, game.board(), *cursor_pos)?;
            draw_info(stdout, game)?;
        }
    }
}

fn draw_board(stdout: &mut Stdout, board: &Board, cursor_pos: (u16, u16)) -> std::io::Result<()> {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
    for r in 0..8 {
        for c in 0..8 {
            let disc = board.get_disc(r, c).unwrap();
            let (bg_color, fg_color) = if (r as u16, c as u16) == cursor_pos {
                (Color::Yellow, get_disc_color(disc))
            } else {
                (Color::DarkGreen, get_disc_color(disc))
            };

            execute!(
                stdout,
                MoveTo(c as u16 * 2 + 2, r as u16 + 1),
                SetBackgroundColor(bg_color),
                SetForegroundColor(fg_color),
                Print(format!("{} ", disc))
            )?;
        }
    }
    execute!(stdout, ResetColor)
}

fn get_disc_color(disc: Disc) -> Color {
    match disc {
        Disc::Black => Color::Black,
        Disc::White => Color::White,
        Disc::Empty => Color::White,
    }
}

fn draw_info(stdout: &mut Stdout, game: &Game) -> std::io::Result<()> {
    let (black_count, white_count) = game.count_discs();
    let current_turn_symbol = if game.current_turn() == Disc::Black { "◯" } else { "●" };
    let turn_color = if game.current_turn() == Disc::Black { Color::White } else { Color::White };
    
    let help_text = "Use arrow keys to move, Enter/Space to place, 'q' to quit.";

    execute!(
        stdout,
        MoveTo(0, 10),
        Print("Turn: "),
        SetForegroundColor(turn_color),
        Print(current_turn_symbol),
        ResetColor,
        MoveTo(0, 11),
        SetForegroundColor(Color::White),
        Print("◯"),
        ResetColor,
        Print(format!(": {} | ", black_count)),
        SetForegroundColor(Color::White),
        Print("●"),
        ResetColor,
        Print(format!(": {}", white_count)),
        MoveTo(0, 12),
        Print(help_text)
    )
}

fn draw_game_mode_selection(stdout: &mut Stdout, error: &str) -> std::io::Result<()> {
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print("Welcome to Othello!"),
        MoveTo(0, 2),
        Print("Select game mode:"),
        MoveTo(2, 3),
        Print("1. Player vs. Player"),
        MoveTo(2, 4),
        Print("2. Player vs. CPU"),
        MoveTo(0, 6),
        Print("Press 'q' to quit."),
        MoveTo(0, 8),
        SetForegroundColor(Color::Red),
        Print(error),
        ResetColor
    )
}

fn draw_cpu_level_selection(stdout: &mut Stdout, error: &str) -> std::io::Result<()> {
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print("Select CPU difficulty:"),
        MoveTo(0, 2),
        Print("1. Easy - Random moves"),
        MoveTo(0, 3),
        Print("2. Medium - Greedy strategy"),
        MoveTo(0, 4),
        Print("3. Hard - Minimax algorithm"),
        MoveTo(0, 6),
        Print("Press 'b' to go back, 'q' to quit."),
        MoveTo(0, 8),
        SetForegroundColor(Color::Red),
        Print(error),
        ResetColor
    )
}

fn draw_game_over(stdout: &mut Stdout, game: &Game) -> std::io::Result<()> {
    let (black_count, white_count) = game.count_discs();
    
    execute!(
        stdout,
        MoveTo(0, 14),
        Print("Game Over!"),
        MoveTo(0, 15),
    )?;

    if black_count > white_count {
        execute!(
            stdout,
            SetForegroundColor(Color::White),
            Print("◯"),
            ResetColor,
            Print(" wins!")
        )?;
    } else if white_count > black_count {
        execute!(
            stdout,
            SetForegroundColor(Color::White),
            Print("●"),
            ResetColor,
            Print(" wins!")
        )?;
    } else {
        execute!(stdout, Print("It's a draw!"))?;
    }

    execute!(
        stdout,
        MoveTo(0, 16),
        Print("Press 'q' to exit.")
    )?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    Ok(())
}

fn show_cpu_thinking(stdout: &mut Stdout, _game: &Game, level: CpuLevel) -> std::io::Result<()> {
    let (thinking_time, frames) = match level {
        CpuLevel::Easy => (800, vec!["CPU is thinking.", "CPU is thinking..", "CPU is thinking..."]),
        CpuLevel::Medium => (1200, vec!["CPU is analyzing.", "CPU is analyzing..", "CPU is analyzing..."]),
        CpuLevel::Hard => (2000, vec!["CPU is calculating.", "CPU is calculating..", "CPU is calculating..."]),
    };
    
    let frame_duration = thinking_time / frames.len() as u64;
    
    for frame in frames {
        execute!(
            stdout,
            MoveTo(0, 13),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::Yellow),
            Print(frame),
            ResetColor
        )?;
        thread::sleep(Duration::from_millis(frame_duration));
    }
    
    // Clear the thinking message
    execute!(
        stdout,
        MoveTo(0, 13),
        Clear(ClearType::CurrentLine)
    )?;
    
    Ok(())
}
