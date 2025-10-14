pub mod game;
pub mod parser;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <sgf-file>", args[0]);
        std::process::exit(1);
    }

    let sgf_path = &args[1];
    let sgf_content = fs::read_to_string(sgf_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read {}: {}", sgf_path, e),
        )
    })?;

    let game_tree = parser::parse_sgf(&sgf_content).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse SGF: {:?}", e),
        )
    })?;

    // Extract board size from properties (default to 19)
    let board_size = game_tree
        .properties
        .get("SZ")
        .and_then(|v| v.first())
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(19);

    let mut game_state =
        game::GameState::with_properties(board_size, game_tree.moves, game_tree.properties);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal, &mut game_state);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    game_state: &mut game::GameState,
) -> io::Result<()> {
    let mut auto_play = true; // Start in play mode by default
    let mut last_auto_advance = std::time::Instant::now();
    let mut playback_speed = 1; // 1x, 2x, or 3x speed

    loop {
        // Calculate delay based on speed: 3000ms at 1x, 1500ms at 2x, 500ms at 3x (6x faster)
        let auto_play_delay = std::time::Duration::from_millis(match playback_speed {
            1 => 3000,
            2 => 1500,
            3 => 500,
            _ => 3000,
        });

        terminal.draw(|f| ui::render_game(f, game_state, auto_play, playback_speed))?;

        // Auto-play logic
        if auto_play && last_auto_advance.elapsed() >= auto_play_delay {
            if !game_state.next() {
                auto_play = false; // Stop at end
            }
            last_auto_advance = std::time::Instant::now();
        }

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char(' ') => {
                        auto_play = !auto_play;
                        last_auto_advance = std::time::Instant::now();
                    }
                    KeyCode::Left => {
                        auto_play = false;
                        game_state.previous();
                    }
                    KeyCode::Right => {
                        auto_play = false;
                        game_state.next();
                    }
                    KeyCode::Home => {
                        auto_play = false;
                        game_state.jump_to_start();
                    }
                    KeyCode::End => {
                        auto_play = false;
                        game_state.jump_to_end();
                    }
                    KeyCode::Char('l') | KeyCode::Char('L') => {
                        game_state.toggle_looping();
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        // Cycle through speeds: 1x -> 2x -> 3x -> 1x
                        playback_speed = if playback_speed >= 3 {
                            1
                        } else {
                            playback_speed + 1
                        };
                    }
                    _ => {}
                }
            }
        }
    }
}
