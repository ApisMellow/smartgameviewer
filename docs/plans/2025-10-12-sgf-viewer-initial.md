# SGF Viewer Initial Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Build a terminal-based Go game viewer that parses SGF files and displays professional games with navigation controls.

**Architecture:** Monolithic modular Rust application with separate modules for SGF parsing, game state management, and Ratatui-based UI. Parser converts SGF to game tree, game module manages board state and move navigation, UI renders board on intersections with keyboard controls.

**Tech Stack:** Rust, Ratatui, Crossterm, devenv for environment management

---

## Task 1: Project Scaffold and DevEnv Setup

**Files:**
- Create: `Cargo.toml`
- Create: `devenv.nix`
- Create: `devenv.yaml`
- Create: `README.md`
- Move: `sgf/AlphaGo_LeeSedol_game4.sgf` → `examples/AlphaGo_LeeSedol_game4.sgf`
- Move: `sgf/AlphaGo_LeeSedol_game5.sgf` → `examples/AlphaGo_LeeSedol_game5.sgf`

**Step 1: Create Cargo.toml with dependencies**

```toml
[package]
name = "smartgameviewer"
version = "0.1.0"
edition = "2021"
authors = ["ApisMellow"]

[dependencies]
ratatui = "0.28"
crossterm = "0.28"

[dev-dependencies]
```

**Step 2: Create devenv.nix**

```nix
{ pkgs, ... }:

{
  packages = [
    pkgs.cargo-watch
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };

  pre-commit.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
```

**Step 3: Create devenv.yaml**

```yaml
inputs:
  nixpkgs:
    url: github:NixOS/nixpkgs/nixpkgs-unstable
```

**Step 4: Create README.md**

```markdown
# Smart Game Viewer

A beautiful terminal-based viewer for SGF (Smart Game Format) files, designed for watching professional Go games.

## Features (v0.1)

- Parse SGF files
- Display Go board (19x19) on terminal with stones on intersections
- Navigate through game moves (forward/backward)
- Auto-play mode
- Example games included (AlphaGo vs Lee Sedol)

## Development

```bash
# Enter development environment
devenv shell

# Run with example game
cargo run -- examples/AlphaGo_LeeSedol_game4.sgf

# Auto-rebuild on changes
cargo watch -x run

# Run tests
cargo test
```

## Roadmap

- [ ] Polish: Colors, gradients, shine effects
- [ ] Sound: Stone placement sounds
- [ ] GUI version (Tauri/egui)
- [ ] Web version
- [ ] AI engine integration

## Author

ApisMellow
```

**Step 5: Move SGF files to examples directory**

```bash
mkdir -p examples
mv sgf/AlphaGo_LeeSedol_game4.sgf examples/
mv sgf/AlphaGo_LeeSedol_game5.sgf examples/
rmdir sgf
```

**Step 6: Initialize Rust project structure**

```bash
mkdir -p src
touch src/main.rs
touch src/parser.rs
touch src/game.rs
touch src/ui.rs
```

**Step 7: Add minimal main.rs**

In `src/main.rs`:

```rust
mod parser;
mod game;
mod ui;

fn main() {
    println!("Smart Game Viewer v0.1.0");
}
```

**Step 8: Verify project compiles**

```bash
cargo build
```

Expected: Successful compilation

**Step 9: Commit project scaffold**

```bash
git add .
git commit -m "feat: initial project scaffold with devenv and Cargo setup

- Add Cargo.toml with ratatui and crossterm dependencies
- Add devenv.nix for Rust development environment
- Add README with project overview and development instructions
- Move SGF examples to examples/ directory
- Create module structure (parser, game, ui)

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 2: SGF Parser - Basic Structure

**Files:**
- Modify: `src/parser.rs`
- Create: `tests/parser_tests.rs`

**Step 1: Write test for parsing empty SGF**

In `tests/parser_tests.rs`:

```rust
use smartgameviewer::parser::*;

#[test]
fn test_parse_empty_game() {
    let sgf = "(;)";
    let result = parse_sgf(sgf);
    assert!(result.is_ok());
    let game = result.unwrap();
    assert_eq!(game.moves.len(), 0);
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_parse_empty_game
```

Expected: Compilation error (parse_sgf doesn't exist)

**Step 3: Write minimal parser types**

In `src/parser.rs`:

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone)]
pub struct Move {
    pub color: Color,
    pub position: Option<(u8, u8)>, // None for pass
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct GameTree {
    pub properties: HashMap<String, Vec<String>>,
    pub moves: Vec<Move>,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat(String),
}

pub fn parse_sgf(input: &str) -> Result<GameTree, ParseError> {
    Ok(GameTree {
        properties: HashMap::new(),
        moves: Vec::new(),
    })
}
```

**Step 4: Make parser module public**

In `src/main.rs`, change:

```rust
pub mod parser;
mod game;
mod ui;
```

**Step 5: Run test to verify it passes**

```bash
cargo test test_parse_empty_game
```

Expected: PASS

**Step 6: Commit parser foundation**

```bash
git add src/parser.rs tests/parser_tests.rs src/main.rs
git commit -m "feat: add basic SGF parser types and structure

- Define Color, Move, GameTree types
- Add ParseError enum for error handling
- Add parse_sgf stub function
- Add test for empty game parsing

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 3: SGF Parser - Property Parsing

**Files:**
- Modify: `src/parser.rs`
- Modify: `tests/parser_tests.rs`

**Step 1: Write test for parsing game properties**

In `tests/parser_tests.rs`:

```rust
#[test]
fn test_parse_game_properties() {
    let sgf = "(;GM[1]FF[4]SZ[19]PB[Lee Sedol]PW[AlphaGo])";
    let result = parse_sgf(sgf);
    assert!(result.is_ok());
    let game = result.unwrap();
    assert_eq!(game.properties.get("SZ").unwrap()[0], "19");
    assert_eq!(game.properties.get("PB").unwrap()[0], "Lee Sedol");
    assert_eq!(game.properties.get("PW").unwrap()[0], "AlphaGo");
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_parse_game_properties
```

Expected: FAIL (properties empty)

**Step 3: Implement property parser**

In `src/parser.rs`, replace `parse_sgf`:

```rust
pub fn parse_sgf(input: &str) -> Result<GameTree, ParseError> {
    let input = input.trim();

    if !input.starts_with('(') || !input.ends_with(')') {
        return Err(ParseError::InvalidFormat("Missing outer parentheses".to_string()));
    }

    // Remove outer parentheses
    let content = &input[1..input.len()-1];

    if !content.starts_with(';') {
        return Err(ParseError::InvalidFormat("Missing initial semicolon".to_string()));
    }

    let mut properties = HashMap::new();
    let moves = Vec::new();

    // Simple property parser - finds KEY[value] patterns
    let mut chars = content.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_uppercase() {
            // Found a property key
            let mut key = String::new();
            key.push(ch);

            // Read rest of key
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_ascii_uppercase() {
                    key.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            // Read value(s) in brackets
            let mut values = Vec::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '[' {
                    chars.next(); // consume '['
                    let mut value = String::new();

                    while let Some(val_ch) = chars.next() {
                        if val_ch == ']' {
                            break;
                        }
                        value.push(val_ch);
                    }

                    values.push(value);
                } else {
                    break;
                }
            }

            if !values.is_empty() {
                properties.insert(key, values);
            }
        }
    }

    Ok(GameTree {
        properties,
        moves,
    })
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_parse_game_properties
```

Expected: PASS

**Step 5: Commit property parsing**

```bash
git add src/parser.rs tests/parser_tests.rs
git commit -m "feat: parse SGF game properties

- Implement property parser for KEY[value] patterns
- Support multiple values per property
- Add validation for SGF format (parentheses, semicolon)
- Add test for parsing game metadata

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 4: SGF Parser - Move Parsing

**Files:**
- Modify: `src/parser.rs`
- Modify: `tests/parser_tests.rs`

**Step 1: Write test for parsing moves**

In `tests/parser_tests.rs`:

```rust
#[test]
fn test_parse_simple_moves() {
    let sgf = "(;GM[1]SZ[19];B[dd];W[pd];B[dp];W[pp])";
    let result = parse_sgf(sgf);
    assert!(result.is_ok());
    let game = result.unwrap();
    assert_eq!(game.moves.len(), 4);
    assert_eq!(game.moves[0].color, Color::Black);
    assert_eq!(game.moves[0].position, Some((3, 3))); // 'dd' = (3,3)
    assert_eq!(game.moves[1].color, Color::White);
    assert_eq!(game.moves[1].position, Some((15, 3))); // 'pd' = (15,3)
}

#[test]
fn test_parse_pass_move() {
    let sgf = "(;GM[1];B[];W[dd])";
    let result = parse_sgf(sgf);
    assert!(result.is_ok());
    let game = result.unwrap();
    assert_eq!(game.moves.len(), 2);
    assert_eq!(game.moves[0].position, None); // Pass
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_parse_simple_moves
```

Expected: FAIL (moves empty)

**Step 3: Add coordinate conversion helper**

In `src/parser.rs`, add:

```rust
// Convert SGF coordinates (e.g., "dd") to board position (3, 3)
// SGF uses 'a' = 0, 'b' = 1, etc.
fn sgf_to_coords(s: &str) -> Option<(u8, u8)> {
    if s.is_empty() {
        return None; // Pass move
    }

    let bytes = s.as_bytes();
    if bytes.len() != 2 {
        return None;
    }

    let col = bytes[0].wrapping_sub(b'a');
    let row = bytes[1].wrapping_sub(b'a');

    if col < 19 && row < 19 {
        Some((col, row))
    } else {
        None
    }
}
```

**Step 4: Extend parser to handle moves**

In `src/parser.rs`, modify `parse_sgf` to parse moves. Replace the function with:

```rust
pub fn parse_sgf(input: &str) -> Result<GameTree, ParseError> {
    let input = input.trim();

    if !input.starts_with('(') || !input.ends_with(')') {
        return Err(ParseError::InvalidFormat("Missing outer parentheses".to_string()));
    }

    // Remove outer parentheses
    let content = &input[1..input.len()-1];

    if !content.starts_with(';') {
        return Err(ParseError::InvalidFormat("Missing initial semicolon".to_string()));
    }

    let mut properties = HashMap::new();
    let mut moves = Vec::new();

    // Split by semicolons to get nodes
    let nodes: Vec<&str> = content.split(';').filter(|s| !s.is_empty()).collect();

    for (idx, node) in nodes.iter().enumerate() {
        let mut chars = node.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch.is_ascii_uppercase() {
                // Found a property key
                let mut key = String::new();
                key.push(ch);

                // Read rest of key
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_uppercase() {
                        key.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                // Read value(s) in brackets
                let mut values = Vec::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '[' {
                        chars.next(); // consume '['
                        let mut value = String::new();

                        while let Some(val_ch) = chars.next() {
                            if val_ch == ']' {
                                break;
                            }
                            value.push(val_ch);
                        }

                        values.push(value);
                    } else {
                        break;
                    }
                }

                if !values.is_empty() {
                    // Check if this is a move property
                    match key.as_str() {
                        "B" => {
                            moves.push(Move {
                                color: Color::Black,
                                position: sgf_to_coords(&values[0]),
                                comment: None,
                            });
                        }
                        "W" => {
                            moves.push(Move {
                                color: Color::White,
                                position: sgf_to_coords(&values[0]),
                                comment: None,
                            });
                        }
                        _ => {
                            // Store as property (only for first node - root properties)
                            if idx == 0 {
                                properties.insert(key, values);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(GameTree {
        properties,
        moves,
    })
}
```

**Step 5: Run tests to verify they pass**

```bash
cargo test
```

Expected: All tests PASS

**Step 6: Commit move parsing**

```bash
git add src/parser.rs tests/parser_tests.rs
git commit -m "feat: parse SGF moves with coordinate conversion

- Parse B[coord] and W[coord] move properties
- Convert SGF coordinates to board positions (0-18)
- Support pass moves (empty brackets)
- Split nodes by semicolon to parse move sequence
- Add tests for move parsing and pass moves

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 5: Game State - Board and Navigation

**Files:**
- Modify: `src/game.rs`
- Create: `tests/game_tests.rs`

**Step 1: Write test for empty board**

In `tests/game_tests.rs`:

```rust
use smartgameviewer::game::*;

#[test]
fn test_empty_board() {
    let board = Board::new(19);
    assert_eq!(board.size, 19);
    assert_eq!(board.get(0, 0), None);
    assert_eq!(board.get(18, 18), None);
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_empty_board
```

Expected: Compilation error (Board doesn't exist)

**Step 3: Implement Board structure**

In `src/game.rs`:

```rust
use crate::parser::{Color, Move};

#[derive(Debug, Clone)]
pub struct Board {
    pub size: u8,
    grid: Vec<Vec<Option<Color>>>,
}

impl Board {
    pub fn new(size: u8) -> Self {
        Board {
            size,
            grid: vec![vec![None; size as usize]; size as usize],
        }
    }

    pub fn get(&self, row: u8, col: u8) -> Option<Color> {
        self.grid[row as usize][col as usize].clone()
    }

    pub fn set(&mut self, row: u8, col: u8, color: Color) {
        self.grid[row as usize][col as usize] = Some(color);
    }

    pub fn clear(&mut self, row: u8, col: u8) {
        self.grid[row as usize][col as usize] = None;
    }
}
```

**Step 4: Make game module public**

In `src/main.rs`:

```rust
pub mod parser;
pub mod game;
mod ui;
```

**Step 5: Run test to verify it passes**

```bash
cargo test test_empty_board
```

Expected: PASS

**Step 6: Write test for game state navigation**

In `tests/game_tests.rs`:

```rust
use smartgameviewer::parser::*;

#[test]
fn test_game_state_navigation() {
    let moves = vec![
        Move { color: Color::Black, position: Some((3, 3)), comment: None },
        Move { color: Color::White, position: Some((15, 3)), comment: None },
        Move { color: Color::Black, position: Some((3, 15)), comment: None },
    ];

    let mut game = GameState::new(19, moves);

    // Start at beginning
    assert_eq!(game.current_move, 0);
    assert_eq!(game.board.get(3, 3), None);

    // Move forward
    game.next();
    assert_eq!(game.current_move, 1);
    assert_eq!(game.board.get(3, 3), Some(Color::Black));

    // Move forward again
    game.next();
    assert_eq!(game.current_move, 2);
    assert_eq!(game.board.get(15, 3), Some(Color::White));

    // Move back
    game.previous();
    assert_eq!(game.current_move, 1);
    assert_eq!(game.board.get(3, 3), Some(Color::Black));
    assert_eq!(game.board.get(15, 3), None);
}
```

**Step 7: Run test to verify it fails**

```bash
cargo test test_game_state_navigation
```

Expected: FAIL (GameState doesn't exist)

**Step 8: Implement GameState**

In `src/game.rs`, add:

```rust
pub struct GameState {
    pub board: Board,
    pub moves: Vec<Move>,
    pub current_move: usize, // 0 = empty board, 1 = after first move, etc.
}

impl GameState {
    pub fn new(board_size: u8, moves: Vec<Move>) -> Self {
        GameState {
            board: Board::new(board_size),
            moves,
            current_move: 0,
        }
    }

    pub fn next(&mut self) -> bool {
        if self.current_move >= self.moves.len() {
            return false; // Already at end
        }

        // Apply the move at current_move index
        if let Some(pos) = self.moves[self.current_move].position {
            self.board.set(pos.0, pos.1, self.moves[self.current_move].color.clone());
        }

        self.current_move += 1;
        true
    }

    pub fn previous(&mut self) -> bool {
        if self.current_move == 0 {
            return false; // Already at start
        }

        self.current_move -= 1;

        // Rebuild board from scratch up to current position
        self.board = Board::new(self.board.size);
        for i in 0..self.current_move {
            if let Some(pos) = self.moves[i].position {
                self.board.set(pos.0, pos.1, self.moves[i].color.clone());
            }
        }

        true
    }

    pub fn jump_to_start(&mut self) {
        self.current_move = 0;
        self.board = Board::new(self.board.size);
    }

    pub fn jump_to_end(&mut self) {
        while self.next() {}
    }
}
```

**Step 9: Run test to verify it passes**

```bash
cargo test test_game_state_navigation
```

Expected: PASS

**Step 10: Commit game state implementation**

```bash
git add src/game.rs tests/game_tests.rs src/main.rs
git commit -m "feat: implement board and game state navigation

- Add Board struct with get/set/clear methods
- Add GameState with move navigation (next, previous)
- Support jump to start/end
- Rebuild board from scratch for backward navigation
- Add comprehensive tests for navigation

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 6: Basic TUI - Render Empty Board

**Files:**
- Modify: `src/ui.rs`
- Modify: `src/main.rs`

**Step 1: Implement basic board rendering**

In `src/ui.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color as RatatuiColor, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};
use crate::game::{Board, GameState};
use crate::parser::Color;

pub fn render_game(frame: &mut Frame, game: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),      // Board
            Constraint::Length(3),  // Status
        ])
        .split(frame.area());

    render_header(frame, chunks[0], game);
    render_board(frame, chunks[1], &game.board);
    render_status(frame, chunks[2], game);
}

fn render_header(frame: &mut Frame, area: Rect, game: &GameState) {
    let text = Line::from(vec![
        Span::raw("Smart Game Viewer"),
    ]);

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(paragraph, area);
}

fn render_board(frame: &mut Frame, area: Rect, board: &Board) {
    let size = board.size as usize;
    let mut lines = Vec::new();

    for row in 0..size {
        let mut line_content = String::new();

        for col in 0..size {
            let stone = match board.get(row as u8, col as u8) {
                Some(Color::Black) => "●",
                Some(Color::White) => "○",
                None => {
                    // Draw intersection
                    if row == 0 && col == 0 {
                        "┌"
                    } else if row == 0 && col == size - 1 {
                        "┐"
                    } else if row == size - 1 && col == 0 {
                        "└"
                    } else if row == size - 1 && col == size - 1 {
                        "┘"
                    } else if row == 0 {
                        "┬"
                    } else if row == size - 1 {
                        "┴"
                    } else if col == 0 {
                        "├"
                    } else if col == size - 1 {
                        "┤"
                    } else {
                        "┼"
                    }
                }
            };

            line_content.push_str(stone);

            // Add horizontal line between intersections (except last column)
            if col < size - 1 {
                line_content.push('─');
            }
        }

        lines.push(Line::from(line_content));
    }

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_status(frame: &mut Frame, area: Rect, game: &GameState) {
    let move_info = format!(
        "Move {}/{} | ← → Step | Space Play/Pause | Q Quit",
        game.current_move,
        game.moves.len()
    );

    let paragraph = Paragraph::new(move_info)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(paragraph, area);
}
```

**Step 2: Implement main event loop**

In `src/main.rs`, replace entire contents:

```rust
pub mod parser;
pub mod game;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::env;
use std::fs;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <sgf-file>", args[0]);
        std::process::exit(1);
    }

    let sgf_path = &args[1];
    let sgf_content = fs::read_to_string(sgf_path)
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("Failed to read {}: {}", sgf_path, e)))?;

    let game_tree = parser::parse_sgf(&sgf_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse SGF: {:?}", e)))?;

    // Extract board size from properties (default to 19)
    let board_size = game_tree.properties.get("SZ")
        .and_then(|v| v.first())
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(19);

    let mut game_state = game::GameState::new(board_size, game_tree.moves);

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

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, game_state: &mut game::GameState) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render_game(f, game_state))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Left => {
                        game_state.previous();
                    }
                    KeyCode::Right => {
                        game_state.next();
                    }
                    KeyCode::Home => {
                        game_state.jump_to_start();
                    }
                    KeyCode::End => {
                        game_state.jump_to_end();
                    }
                    _ => {}
                }
            }
        }
    }
}
```

**Step 3: Test manually with example SGF**

```bash
cargo run -- examples/AlphaGo_LeeSedol_game4.sgf
```

Expected: Terminal UI appears with Go board, use arrow keys to navigate

**Step 4: Verify navigation works**

- Press Right arrow: should advance moves
- Press Left arrow: should go back
- Press Home: should go to start
- Press End: should go to end
- Press Q: should quit

**Step 5: Commit basic TUI**

```bash
git add src/ui.rs src/main.rs
git commit -m "feat: implement basic TUI with board rendering and navigation

- Render 19x19 Go board with Unicode box-drawing characters
- Display stones on intersections (● for black, ○ for white)
- Add header, board, and status bar layout
- Implement keyboard navigation (←/→ for moves, Home/End for jumps)
- Load SGF from command line argument
- Handle graceful terminal setup/teardown

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 7: Add Auto-Play Feature

**Files:**
- Modify: `src/main.rs`

**Step 1: Add auto-play state**

In `src/main.rs`, modify `run_app` function to include auto-play:

```rust
fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, game_state: &mut game::GameState) -> io::Result<()> {
    let mut auto_play = false;
    let mut last_auto_advance = std::time::Instant::now();
    let auto_play_delay = std::time::Duration::from_millis(1500); // 1.5 seconds per move

    loop {
        terminal.draw(|f| ui::render_game(f, game_state))?;

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
                    _ => {}
                }
            }
        }
    }
}
```

**Step 2: Update status bar to show auto-play state**

In `src/ui.rs`, modify `render_status`:

```rust
pub fn render_game(frame: &mut Frame, game: &GameState, auto_play: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),      // Board
            Constraint::Length(3),  // Status
        ])
        .split(frame.area());

    render_header(frame, chunks[0], game);
    render_board(frame, chunks[1], &game.board);
    render_status(frame, chunks[2], game, auto_play);
}

fn render_status(frame: &mut Frame, area: Rect, game: &GameState, auto_play: bool) {
    let play_status = if auto_play { "[PLAYING]" } else { "[PAUSED]" };
    let move_info = format!(
        "Move {}/{} {} | ← → Step | Space Play/Pause | Q Quit",
        game.current_move,
        game.moves.len(),
        play_status
    );

    let paragraph = Paragraph::new(move_info)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(paragraph, area);
}
```

**Step 3: Update main.rs render call**

In `src/main.rs`, update the render call in `run_app`:

```rust
terminal.draw(|f| ui::render_game(f, game_state, auto_play))?;
```

**Step 4: Test auto-play manually**

```bash
cargo run -- examples/AlphaGo_LeeSedol_game4.sgf
```

Expected: Press Space to start auto-play, moves advance automatically

**Step 5: Commit auto-play feature**

```bash
git add src/main.rs src/ui.rs
git commit -m "feat: add auto-play feature

- Toggle auto-play with spacebar
- Advance moves automatically every 1.5 seconds
- Stop auto-play at end of game or on manual navigation
- Show [PLAYING]/[PAUSED] status in status bar

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 8: Polish - Display Game Information

**Files:**
- Modify: `src/ui.rs`
- Modify: `src/game.rs`

**Step 1: Add game metadata to GameState**

In `src/game.rs`, modify `GameState`:

```rust
use std::collections::HashMap;

pub struct GameState {
    pub board: Board,
    pub moves: Vec<Move>,
    pub current_move: usize,
    pub properties: HashMap<String, Vec<String>>, // Game metadata
}

impl GameState {
    pub fn new(board_size: u8, moves: Vec<Move>) -> Self {
        GameState {
            board: Board::new(board_size),
            moves,
            current_move: 0,
            properties: HashMap::new(),
        }
    }

    pub fn with_properties(board_size: u8, moves: Vec<Move>, properties: HashMap<String, Vec<String>>) -> Self {
        GameState {
            board: Board::new(board_size),
            moves,
            current_move: 0,
            properties,
        }
    }

    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).and_then(|v| v.first()).map(|s| s.as_str())
    }

    // ... rest of methods unchanged
}
```

**Step 2: Update main.rs to pass properties**

In `src/main.rs`, change game state creation:

```rust
let mut game_state = game::GameState::with_properties(board_size, game_tree.moves, game_tree.properties);
```

**Step 3: Enhance header display**

In `src/ui.rs`, update `render_header`:

```rust
fn render_header(frame: &mut Frame, area: Rect, game: &GameState) {
    let black_player = game.get_property("PB").unwrap_or("Black");
    let white_player = game.get_property("PW").unwrap_or("White");
    let game_name = game.get_property("GN").unwrap_or("Go Game");

    let text = Line::from(vec![
        Span::raw(format!("{} | ", game_name)),
        Span::styled(black_player, Style::default().fg(RatatuiColor::White)),
        Span::raw(" vs "),
        Span::styled(white_player, Style::default().fg(RatatuiColor::Gray)),
    ]);

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(paragraph, area);
}
```

**Step 4: Show current move information**

In `src/ui.rs`, update `render_status` to show current move details:

```rust
fn render_status(frame: &mut Frame, area: Rect, game: &GameState, auto_play: bool) {
    let play_status = if auto_play { "[PLAYING]" } else { "[PAUSED]" };

    let current_move_info = if game.current_move > 0 && game.current_move <= game.moves.len() {
        let mv = &game.moves[game.current_move - 1];
        let color = match mv.color {
            crate::parser::Color::Black => "Black",
            crate::parser::Color::White => "White",
        };
        let pos = if let Some((row, col)) = mv.position {
            format!("{}{}", (b'A' + col) as char, row + 1)
        } else {
            "Pass".to_string()
        };
        format!(" | {} {}", color, pos)
    } else {
        "".to_string()
    };

    let move_info = format!(
        "Move {}/{}{} {} | ← → Step | Space Play/Pause | Q Quit",
        game.current_move,
        game.moves.len(),
        current_move_info,
        play_status
    );

    let paragraph = Paragraph::new(move_info)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(paragraph, area);
}
```

**Step 5: Test with real game file**

```bash
cargo run -- examples/AlphaGo_LeeSedol_game4.sgf
```

Expected: Header shows player names, status shows move coordinates

**Step 6: Commit game information display**

```bash
git add src/game.rs src/ui.rs src/main.rs
git commit -m "feat: display game information and current move details

- Show game name and player names in header
- Display current move coordinates in status bar
- Add properties to GameState for metadata access
- Convert board coordinates to readable format (A1, B2, etc.)

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 9: Test with Real SGF Files

**Files:**
- None (manual testing)

**Step 1: Test with AlphaGo game 4**

```bash
cargo run -- examples/AlphaGo_LeeSedol_game4.sgf
```

Verify:
- Game loads successfully
- Board displays correctly
- All moves can be navigated
- Auto-play works
- Player names show correctly

**Step 2: Test with AlphaGo game 5**

```bash
cargo run -- examples/AlphaGo_LeeSedol_game5.sgf
```

Verify same functionality

**Step 3: Test edge cases**

- Navigate to end, press Right (should not crash)
- Navigate to start, press Left (should not crash)
- Start auto-play at end (should not advance)
- Quit during auto-play (should exit cleanly)

**Step 4: Document any issues found**

If issues found, create additional tasks to fix them. Otherwise, proceed.

---

## Task 10: Final Polish and Documentation

**Files:**
- Modify: `README.md`
- Create: `LICENSE`

**Step 1: Update README with usage instructions**

In `README.md`, add:

```markdown
## Usage

```bash
# Run with an SGF file
cargo run -- examples/AlphaGo_LeeSedol_game4.sgf

# Or after building
./target/release/smartgameviewer examples/AlphaGo_LeeSedol_game4.sgf
```

## Controls

- **← / →**: Step backward/forward through moves
- **Home**: Jump to start of game
- **End**: Jump to end of game
- **Space**: Toggle auto-play (automatically advance moves)
- **Q / Esc**: Quit

## Features

- Parse and display SGF (Smart Game Format) files
- Navigate through game moves with keyboard controls
- Auto-play mode for watching games
- Display game information (players, move coordinates)
- Support for 9x9, 13x13, and 19x19 boards
- Clean Unicode-based board rendering

## Development

```bash
# Enter development shell
devenv shell

# Run with example
cargo run -- examples/AlphaGo_LeeSedol_game4.sgf

# Watch for changes and auto-rebuild
cargo watch -x "run -- examples/AlphaGo_LeeSedol_game4.sgf"

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```
```

**Step 2: Add MIT license**

In `LICENSE`:

```
MIT License

Copyright (c) 2025 ApisMellow

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

**Step 3: Add license to Cargo.toml**

In `Cargo.toml`:

```toml
[package]
name = "smartgameviewer"
version = "0.1.0"
edition = "2021"
authors = ["ApisMellow"]
license = "MIT"
description = "A beautiful terminal-based viewer for SGF Go game files"
repository = "https://github.com/ApisMellow/smartgameviewer"
```

**Step 4: Commit documentation**

```bash
git add README.md LICENSE Cargo.toml
git commit -m "docs: add comprehensive README and MIT license

- Add detailed usage instructions
- Document keyboard controls
- Add development workflow
- Add MIT license
- Update Cargo.toml with license and repository info

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Step 5: Final verification**

```bash
cargo test
cargo clippy
cargo fmt --check
```

Expected: All pass

**Step 6: Create release build**

```bash
cargo build --release
./target/release/smartgameviewer examples/AlphaGo_LeeSedol_game4.sgf
```

Expected: Fast, smooth execution

---

## Completion Checklist

- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Manual testing complete with both SGF files
- [ ] README accurate and complete
- [ ] Ready to merge to main branch

## Future Enhancements (Not in this plan)

- Add colors and gradients to stones
- Implement shine/gloss effects
- Add star points (hoshi) display
- Support for game variations/branches
- Sound effects for stone placement
- Export board position as image
- GUI version with Tauri/egui
- Web version with WASM
- AI engine integration
