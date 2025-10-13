# Smart Game Viewer

A beautiful terminal-based viewer for SGF (Smart Game Format) files, designed for watching professional Go games.

## Features

- Parse and display SGF (Smart Game Format) files
- Navigate through game moves with keyboard controls
- Auto-play mode for watching games
- Display game information (players, move coordinates)
- Support for 9x9, 13x13, and 19x19 boards
- Clean Unicode-based board rendering with stones on intersections
- Example games included (AlphaGo vs Lee Sedol)

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

## Development

```bash
# Enter development environment
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

## Roadmap

- [ ] Polish: Colors, gradients, shine effects
- [ ] Sound: Stone placement sounds
- [ ] GUI version (Tauri/egui)
- [ ] Web version
- [ ] AI engine integration

## License

MIT

## Author

ApisMellow
