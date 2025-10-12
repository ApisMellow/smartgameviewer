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
