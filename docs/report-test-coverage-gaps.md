# Test Coverage Gap Report

Generated: 2026-03-04

This report identifies missing test coverage across the smartgameviewer codebase. Use it to prioritize and write new tests.

---

## Current State

27 tests pass across 7 test files. Coverage is strong for happy-path navigation and parsing but weak on error paths, boundary conditions, and two entire modules.

---

## 1. Untested Modules

### ui.rs — No test coverage at all

All render functions (`render_game`, `render_header`, `render_board`, `render_status`, `render_transition`) are private to the binary. They are not re-exported through `lib.rs`, so integration tests cannot reach them.

**To fix:** Either expose render functions through `lib.rs` behind a `#[cfg(test)]` gate, or create a test helper that provides a buffer-backed Ratatui terminal.

### main.rs — Not testable

`load_game_from_path` and `run_app` are private binary functions. Not a priority to unit-test, but `load_game_from_path` logic (SZ extraction, fallback to 19) could be extracted into a testable function in `lib.rs`.

### ui_tests.rs — Misnamed

`tests/ui_tests.rs` does not test `ui.rs`. It tests `Board` and `GameState` logic, duplicating coverage from `game_tests.rs` and `integration_tests.rs`. Consider renaming it to `board_state_tests.rs` or removing it and merging its assertions into existing test files.

---

## 2. Untested Public API Methods

These are `pub` methods exported from `lib.rs` with zero test coverage:

| Method | Module | Used in production? | Notes |
|--------|--------|---------------------|-------|
| `Board::clear(row, col)` | `game.rs` | No | Dead code — never called anywhere |
| `GameState::with_properties(size, moves, props)` | `game.rs` | Yes (`main.rs`) | The constructor used by the actual app; tests only use `GameState::new` |
| `GameState::get_property(key)` | `game.rs` | Yes (`ui.rs`, `main.rs`) | Used for title, player names; never tested |
| `BoardView::size()` | `board_view.rs` | No | Accessor, low priority |
| `PlaylistManager::peek_next()` | `playlist.rs` | Yes (`main.rs`) | Used for transition screen |
| `PlaylistManager::is_empty()` | `playlist.rs` | Conditionally | Only tested when `./sgf/` happens to exist |

### Recommended tests

```
test_with_properties_and_get_property
  - Construct GameState via with_properties with known map
    e.g. {"PB": ["Alice"], "GN": ["My Game"]}
  - Assert get_property("PB") == Some("Alice")
  - Assert get_property("MISSING") == None

test_peek_next
  - Create a 3-file playlist
  - Assert peek_next() returns Some(second file) without advancing
  - Call next(), assert peek_next() returns Some(third file)
  - Call next(), assert peek_next() returns None

test_board_clear
  - Set a stone, call clear(), assert get() returns None
  - Or: remove Board::clear entirely if it's dead code
```

---

## 3. Untested Error Paths

| Error | Location | What to test |
|-------|----------|--------------|
| `ParseError::InvalidFormat("Missing outer parentheses")` | `parser.rs` | Input: `"no parens"` → expect `Err` |
| `ParseError::InvalidFormat("Missing initial semicolon")` | `parser.rs` | Input: `"(no semicolon)"` → expect `Err` |
| `io::Error(NotFound, "Path not found: ...")` | `playlist.rs` | `PlaylistManager::new(Some("/nonexistent"))` → expect `Err` |
| `io::Error(NotFound, "No SGF files found")` | `playlist.rs` | `PlaylistManager::new(Some(empty_dir))` → expect `Err` |
| `parse_sgf` with empty string | `parser.rs` | Input: `""` → expect `Err` |

### Recommended tests

```
test_parse_sgf_missing_parentheses
  - parse_sgf("no parens") → assert Err

test_parse_sgf_missing_semicolon
  - parse_sgf("(no semicolon)") → assert Err

test_parse_sgf_empty_input
  - parse_sgf("") → assert Err

test_playlist_nonexistent_path
  - PlaylistManager::new(Some("/tmp/does_not_exist_xyz")) → assert Err

test_playlist_empty_directory
  - Create a tempdir with no .sgf files
  - PlaylistManager::new(Some(tempdir_path)) → assert Err
```

---

## 4. Untested Edge Cases

### game.rs

| Edge case | What to test |
|-----------|--------------|
| `previous()` at move 0 | Should return `false`, board unchanged |
| `next()` with pass move (`position: None`) | `current_move` increments, no stone placed on board |
| `jump_to_end()` with zero moves | `current_move` stays 0, board empty |
| `jump_to_start()` when already at start | Idempotent — no state change |
| Non-19×19 board | Create `Board::new(9)`, navigate a game on it |

### board_view.rs

| Edge case | What to test |
|-----------|--------------|
| Rotation 90° (rotation=1) | Place stone at `(3, 5)`, verify view maps to `(5, size-1-3)` = `(5, 15)` |
| Rotation 270° (rotation=3) | Place stone at `(3, 5)`, verify view maps to `(size-1-5, 3)` = `(13, 3)` |
| `get()` returning `None` through view | Assert empty cell returns `None` via `BoardView` |
| Non-19×19 board in a view | `BoardView` over a 9×9 board |

### parser.rs

| Edge case | What to test |
|-----------|--------------|
| `sgf_to_coords` with 1-char string | Returns `None` |
| `sgf_to_coords` with 3-char string | Returns `None` |
| Coords at boundary (col or row = 19) | e.g. `"ta"` → should return `None` (>= 19 check) |
| Leading/trailing whitespace | `"  (;GM[1])  "` → parses successfully |
| Multi-value properties `AB[dd][ee]` | Values vec has 2 entries |
| Non-root node properties ignored | `"(;GM[1];B[dd]SZ[9])"` → properties should NOT contain "SZ" |

### playlist.rs

| Edge case | What to test |
|-----------|--------------|
| `next()` returns `false` at end | Exhaust playlist, assert final `next()` returns `false` |
| Case-insensitive `.SGF` extension | File with `.SGF` or `.Sgf` extension should be discovered |
| `tokenize` edge inputs | Purely numeric `"123"`, purely alpha `"abc"`, empty string |

---

## 5. Priority Order for Implementation

**High — public API gaps used in production:**
1. `with_properties` + `get_property` round-trip
2. `previous()` at boundary (move 0)
3. `next()` with pass moves
4. `peek_next()` and `next()` returning `false`
5. Parser error paths (all 3)
6. Playlist error paths (both)

**Medium — correctness edge cases:**
7. 90° and 270° rotation in `BoardView`
8. Non-19×19 board sizes
9. `jump_to_end` with zero moves
10. `sgf_to_coords` boundary and malformed inputs
11. Multi-value properties
12. Non-root node property suppression

**Low — cleanup and completeness:**
13. `Board::clear` (or remove if dead code)
14. `BoardView::size()`
15. Case-insensitive extension matching
16. `tokenize` edge inputs
17. Rename or repurpose `ui_tests.rs`
18. Explore making `ui.rs` testable via `lib.rs`
