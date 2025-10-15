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

    loop {
        match &mut app_state {
            AppState::Playing {
                game,
                auto_play,
                playback_speed,
                last_auto_advance,
            } => {
                // Calculate delay based on speed
                let auto_play_delay = std::time::Duration::from_millis(match *playback_speed {
                    1 => 3000,
                    2 => 1500,
                    3 => 500,
                    _ => 3000,
                });

                terminal.draw(|f| ui::render_game(f, game, *auto_play, *playback_speed))?;

                // Auto-play logic
                if *auto_play && last_auto_advance.elapsed() >= auto_play_delay {
                    if game.current_move >= game.moves.len() {
                        // Reached end of current game
                        if playlist.has_next() {
                            // Transition to next file
                            let from_title = game.get_property("GN").unwrap_or("Game").to_string();

                            if let Some(next_path) = playlist.peek_next() {
                                match load_game_from_path(next_path) {
                                    Ok(next_game) => {
                                        let to_title = next_game
                                            .get_property("GN")
                                            .unwrap_or("Game")
                                            .to_string();
                                        playlist.next();

                                        app_state = AppState::Transition {
                                            from_title,
                                            to_title,
                                            start_time: Instant::now(),
                                        };
                                        continue;
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to load next game: {}", e);
                                        *auto_play = false;
                                    }
                                }
                            }
                        } else if game.is_looping_enabled() && !playlist.is_single_file() {
                            // Last file, loop back to first
                            let from_title = game.get_property("GN").unwrap_or("Game").to_string();
                            playlist.reset();

                            match load_game_from_path(playlist.current()) {
                                Ok(first_game) => {
                                    let to_title =
                                        first_game.get_property("GN").unwrap_or("Game").to_string();

                                    app_state = AppState::Transition {
                                        from_title,
                                        to_title,
                                        start_time: Instant::now(),
                                    };
                                    continue;
                                }
                                Err(e) => {
                                    eprintln!("Failed to reload first game: {}", e);
                                    *auto_play = false;
                                }
                            }
                        } else {
                            // Last file + no loop OR single file
                            // Let game handle its own looping
                            let can_continue = game.next();
                            if !can_continue {
                                *auto_play = false;
                            }
                        }
                    } else {
                        // Normal move advancement
                        game.next();
                    }
                    *last_auto_advance = Instant::now();
                }

                // Event handling
                if event::poll(std::time::Duration::from_millis(100))? {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char(' ') => {
                                *auto_play = !*auto_play;
                                *last_auto_advance = Instant::now();
                            }
                            KeyCode::Left => {
                                *auto_play = false;
                                game.previous();
                            }
                            KeyCode::Right => {
                                *auto_play = false;
                                game.next();
                            }
                            KeyCode::Home => {
                                *auto_play = false;
                                game.jump_to_start();
                            }
                            KeyCode::End => {
                                *auto_play = false;
                                game.jump_to_end();
                            }
                            KeyCode::Char('l') | KeyCode::Char('L') => {
                                game.toggle_looping();
                            }
                            KeyCode::Char('s') | KeyCode::Char('S') => {
                                *playback_speed = if *playback_speed >= 3 {
                                    1
                                } else {
                                    *playback_speed + 1
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
            AppState::Transition { .. } => {
                // Placeholder - will implement in Task 5
                break;
            }
        }
    }

    Ok(())
}
