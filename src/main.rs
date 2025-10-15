pub mod board_view;
pub mod game;
pub mod parser;
pub mod playlist;
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

use std::time::Instant;

use playlist::PlaylistManager;

enum AppState {
    Playing {
        game: game::GameState,
        auto_play: bool,
        playback_speed: u64,
        last_auto_advance: Instant,
    },
    Transition {
        from_title: String,
        to_title: String,
        start_time: Instant,
    },
}

fn load_game_from_path(path: &std::path::Path) -> Result<game::GameState, io::Error> {
    let sgf_content = fs::read_to_string(path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read {}: {}", path.display(), e),
        )
    })?;

    let game_tree = parser::parse_sgf(&sgf_content).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse SGF: {:?}", e),
        )
    })?;

    let board_size = game_tree
        .properties
        .get("SZ")
        .and_then(|v| v.first())
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(19);

    Ok(game::GameState::with_properties(
        board_size,
        game_tree.moves,
        game_tree.properties,
    ))
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let path_arg = args.get(1).map(|s| s.as_str());

    let playlist = PlaylistManager::new(path_arg).map_err(|e| {
        if path_arg.is_none() {
            io::Error::new(
                io::ErrorKind::NotFound,
                "No SGF files found in ./sgf/ folder. Place .sgf files there or specify a file path.",
            )
        } else {
            e
        }
    })?;

    let initial_game = load_game_from_path(playlist.current())?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal, initial_game, playlist);

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
    initial_game: game::GameState,
    mut playlist: PlaylistManager,
) -> io::Result<()> {
    let mut app_state = AppState::Playing {
        game: initial_game,
        auto_play: true,
        playback_speed: 1,
        last_auto_advance: Instant::now(),
    };

    // Placeholder - will implement in Task 4
    #[allow(clippy::todo)]
    todo!("Implement main loop in Task 4");
}
