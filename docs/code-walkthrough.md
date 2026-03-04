# Smart Game Viewer — Code Walkthrough

A terminal-based Go game viewer built with Rust, Ratatui, and Crossterm. This document walks through the codebase from startup to rendering, explaining the flow and notable techniques.

## Architecture at a Glance

```
src/
├── main.rs        Entry point, event loop, state machine
├── lib.rs         Public module exports
├── parser.rs      SGF text → GameTree data structure
├── game.rs        Board state, move navigation, looping
├── board_view.rs  Zero-copy rotated board view
├── ui.rs          Ratatui rendering and animations
└── playlist.rs    Multi-file management with natural sort
```

Data flows in one direction:

```
SGF file → parser → GameTree → GameState → BoardView → UI
```

---

## Startup Flow (main.rs)

1. **Resolve input.** CLI arg can be a single `.sgf` file, a directory of them, or nothing (defaults to `./sgf/`).
2. **Build playlist.** `PlaylistManager` collects and naturally sorts all discovered files.
3. **Load first game.** Read the file, parse SGF, extract board size, create `GameState`.
4. **Set up terminal.** Enable raw mode, enter alternate screen, enable mouse capture, create Ratatui `Terminal`.
5. **Enter event loop** via `run_app()`.

---

## The Event Loop and State Machine

The application uses an enum-based state machine with two variants:

```rust
enum AppState {
    Playing {
        game: GameState,
        auto_play: bool,
        playback_speed: u64,   // 1, 2, or 3
        last_auto_advance: Instant,
    },
    Transition {
        from_title: String,
        to_title: String,
        start_time: Instant,
    },
}
```

### Playing state

Each iteration:

1. Render the current board.
2. If auto-play is on and enough time has elapsed (3000ms / 1500ms / 500ms for speeds 1x/2x/3x), advance one move.
3. Poll for keyboard input with a 100ms timeout.
4. Dispatch the key: arrow keys for navigation, space for play/pause, `l` for loop toggle, `s` for speed cycling, `q`/Esc to quit.

When the game reaches its last move during auto-play:
- If the playlist has more files → enter Transition state.
- If looping is enabled → rotate the board 180° and restart.

### Transition state

Displays "Next: [Game Title]" with a pulsing orange/yellow animation for 3 seconds, then loads the next game. The player can interrupt with `q`/Esc.

Pattern matching on `AppState` keeps the two modes cleanly separated — no nested conditionals, no boolean flags to track which mode we're in.

---

## SGF Parser (parser.rs)

### Data types

```rust
pub enum Color { Black, White }

pub struct Move {
    pub color: Color,
    pub position: Option<(u8, u8)>,  // None = pass
    pub comment: Option<String>,
}

pub struct GameTree {
    pub properties: HashMap<String, Vec<String>>,
    pub moves: Vec<Move>,
}
```

### Parsing algorithm

`parse_sgf(input: &str) -> Result<GameTree>` works in one pass:

1. **Validate structure** — must start with `(`, end with `)`, contain `;`.
2. **Split by semicolons** to get nodes.
3. **Parse each node** character-by-character using a `Peekable` iterator:
   - Uppercase letters accumulate into a property key.
   - `[...]` brackets delimit values.
   - Multiple `[value1][value2]` blocks are collected into a `Vec`.
4. **Classify properties** — `B` and `W` keys become `Move`s; everything else is metadata.

### Coordinate conversion

SGF encodes positions as two lowercase letters: `"pd"` means column `p`, row `d`.

```rust
fn sgf_to_coords(s: &str) -> Option<(u8, u8)> {
    let col = bytes[0].wrapping_sub(b'a');  // 'p' → 15
    let row = bytes[1].wrapping_sub(b'a');  // 'd' → 3
    Some((row, col))                        // (3, 15) in row-major order
}
```

Empty brackets `B[]` indicate a pass move → `position: None`.

**Technique: Peekable iterator.** The parser uses `chars.peek()` to look ahead without consuming, which lets it distinguish between multi-character property keys and the start of a value bracket in a single forward pass.

---

## Game State (game.rs)

### Board

```rust
pub struct Board {
    size: u8,
    grid: Vec<Vec<Option<Color>>>,
}
```

A simple 2D grid. `get(row, col)` returns the stone color; `set` and `clear` mutate it. No capture logic — this is a viewer, not an engine.

### Navigation

```rust
pub struct GameState {
    board: Board,
    moves: Vec<Move>,
    current_move: usize,
    properties: HashMap<String, Vec<String>>,
    looping_enabled: bool,
    rotation: u8,  // 0, 1, 2, or 3
}
```

- **Forward (`next()`):** Apply `moves[current_move]` to the board, increment index. O(1).
- **Backward (`previous()`):** Decrement index, rebuild the entire board by replaying moves 0 through `current_move - 1`. O(n).
- **Jump to start/end:** Clear board or replay all moves.

**Technique: Rebuild instead of undo stack.** Going backward replays the game from scratch rather than maintaining an undo stack. This trades a bit of CPU (replaying up to ~300 moves on a 19×19 board) for much simpler code — no undo/redo bookkeeping, no risk of state divergence. The cost is negligible for boards this size.

### Looping with rotation

When `next()` is called at the end of the game with looping enabled:

```rust
self.rotation = (self.rotation + 2) % 4;  // +180°
self.jump_to_start();
```

The game restarts from an empty board, but the UI now shows it rotated 180°. This gives the viewer a fresh perspective on each loop. The rotation cycles 0° → 180° → 0° → 180°.

---

## Board View (board_view.rs)

```rust
pub struct BoardView<'a> {
    board: &'a Board,
    rotation: u8,
}
```

**Technique: Zero-copy view with lifetime-bounded borrow.** `BoardView` holds a shared reference to the board, not a clone. The lifetime `'a` guarantees the view cannot outlive the board it references. No memory is allocated.

Rotation is applied on read via coordinate transformation:

| Rotation | Mapping `(r, c) →`         |
|----------|-----------------------------|
| 0°       | `(r, c)`                    |
| 90°      | `(c, size-1-r)`             |
| 180°     | `(size-1-r, size-1-c)`      |
| 270°     | `(size-1-c, r)`             |

This keeps rotation as pure presentation logic. The underlying `Board` is never mutated for display purposes — the `GameState` stores rotation as metadata, and `BoardView` interprets it at render time.

---

## UI Rendering (ui.rs)

### Layout

The terminal is split into four vertical sections:

```
┌──────────────────────────┐
│  Header (3 lines)        │  Game title + player names
├──────────────────────────┤
│                          │
│  Board (size + 2 lines)  │  19×19 grid with stones
│                          │
├──────────────────────────┤
│  Filler (flexible)       │  Absorbs remaining space
├──────────────────────────┤
│  Status (3 lines)        │  Move info, controls, animations
└──────────────────────────┘
```

### Board rendering

The board uses Unicode box-drawing characters for the grid and emoji for stones:

- `⚫` Black stone, `⚪` White stone
- `┌─┬┐` top edge, `├─┼┤` middle, `└─┴┘` bottom edge
- Background: tan wood color (RGB 210, 180, 140)

A `BoardView` is created with the current rotation, then iterated row-by-row to produce styled `Span`s.

### Animations

All animations are time-based using `SystemTime::now()` — no threads, no async runtime.

**Title shine effect.** A 3-character bright gradient sweeps across the game title:
- Center character: bold, brightest (255, 240, 210)
- Adjacent characters: medium brightness (255, 210, 165)
- Base text: pale orange (255, 190, 140)
- Sweeps right then left, 150ms per position

**Star speed indicator (★).** A 10-frame animation cycle in the status bar:
| Frame | Symbol | Style |
|-------|--------|-------|
| 0 | `·` | gray |
| 1 | `∙` | slightly brighter |
| 2 | `+` | dim yellow |
| 3 | `✢` | medium bright |
| 4 | `*` | bright yellow |
| 5 | `✦` | very bright, bold |
| 6 | `★` | peak brightness, bold |
| 7 | `✦` | bright, bold |
| 8 | `*` | bright |
| 9 | `+` | fading |

Animation speed scales with playback speed.

**Transition pulse.** The "Next: [Title]" text pulses between orange and yellow using a sine wave on the elapsed time.

**Technique: Deterministic time-based animation.** Frame position is computed from `elapsed_ms % cycle_length`, making animations purely a function of wall-clock time. No frame counters to manage, no drift, no thread synchronization.

---

## Playlist Manager (playlist.rs)

### File discovery

Given a path, the playlist manager:
- If it's a file → single-file playlist.
- If it's a directory → scan for all `.sgf` files (case-insensitive extension match).
- If no path → scan `./sgf/`.

### Natural sort

Files are sorted using a tokenization strategy so that `game2.sgf` sorts before `game10.sgf`:

```
"game10.sgf" → [Text("game"), Number(10), Text(".sgf")]
"game2.sgf"  → [Text("game"), Number(2),  Text(".sgf")]
```

```rust
#[derive(Ord, PartialOrd)]
enum Token {
    Text(String),
    Number(usize),
}
```

Text tokens compare lexicographically; Number tokens compare numerically. Rust's derived `Ord` on the enum handles the comparison automatically since `Text` is the first variant.

### Navigation API

```rust
playlist.current()     // → &Path
playlist.next()        // → bool (advance)
playlist.peek_next()   // → Option<&Path>
playlist.has_next()    // → bool
playlist.reset()       // → back to first file
playlist.is_single_file() // → bool
```

When the playlist reaches the end and looping is enabled, `reset()` wraps back to the first file.

---

## Key Rust Patterns

| Pattern | Where | Why |
|---------|-------|-----|
| Enum state machine | `AppState` in main.rs | Clean state transitions via pattern matching, no invalid states |
| Borrowed view with lifetime | `BoardView<'a>` | Zero-copy rotation without cloning the board |
| `Option<T>` for nullable values | `Move.position` | Pass moves are `None`, not a sentinel value |
| `Result<T, E>` for errors | `parse_sgf()` return | Caller must handle parse failures explicitly |
| `HashMap<String, Vec<String>>` | Game properties | Matches SGF's multi-value property model |
| Peekable iterator | Parser character loop | Look-ahead without consuming input |
| Derived trait implementations | `Token` in playlist.rs | `#[derive(Ord)]` gives natural sort for free |
| Modular arithmetic | Rotation cycling | `(rotation + 2) % 4` wraps cleanly |

---

## Test Organization

Tests live in `tests/` as integration tests (they import `smartgameviewer` as a library via `lib.rs`):

| File | Tests | Focus |
|------|-------|-------|
| `parser_tests.rs` | 4 | Empty games, properties, moves, passes |
| `game_tests.rs` | 9 | Navigation, looping, rotation, coordinates |
| `board_view_tests.rs` | 3 | Rotation transforms at 0° and 180° |
| `ui_tests.rs` | 3 | Board rendering with and without stones |
| `integration_tests.rs` | 2 | Loading real AlphaGo SGF files end-to-end |
| `playlist_tests.rs` | 4 | Single file, folder scan, natural sort, reset |
| `playlist_integration_tests.rs` | 2 | Playlist with real SGF files on disk |

All 27 tests pass. The real SGF files in `examples/` and `test_sgf/` are used by integration tests to verify the full pipeline from file read to game state.
