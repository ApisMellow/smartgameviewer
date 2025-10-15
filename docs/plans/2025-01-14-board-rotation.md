# Board Rotation on Loop Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Rotate the board view 180° each time the game loops back to the start, giving viewers a different perspective as if sitting on the opposite side of the board.

**Architecture:** Create a `BoardView` struct that wraps Board with rotation state (0°, 90°, 180°, 270°). All coordinate transformations are handled in BoardView::get(). GameState tracks rotation and increments it by 180° when looping. Rendering uses BoardView instead of direct Board access.

**Tech Stack:** Rust, ratatui TUI framework, existing game logic in src/game.rs

---

## Task 1: Create BoardView Module with Rotation Logic

**Files:**
- Create: `src/board_view.rs`
- Modify: `src/lib.rs` (add module export)
- Test: `tests/board_view_tests.rs`

**Step 1: Write failing test for 0° rotation**

Create `tests/board_view_tests.rs`:

```rust
use smartgameviewer::board_view::BoardView;
use smartgameviewer::game::Board;
use smartgameviewer::parser::Color;

#[test]
fn test_rotation_0_degrees() {
    let mut board = Board::new(19);
    board.set(0, 0, Color::Black);
    board.set(18, 18, Color::White);

    let view = BoardView::new(&board, 0);

    assert_eq!(view.get(0, 0), Some(Color::Black));
    assert_eq!(view.get(18, 18), Some(Color::White));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_rotation_0_degrees`

Expected: Compilation error "no module named board_view"

**Step 3: Create BoardView struct**

Create `src/board_view.rs`:

```rust
use crate::game::Board;
use crate::parser::Color;

pub struct BoardView<'a> {
    board: &'a Board,
    rotation: u8,
}

impl<'a> BoardView<'a> {
    pub fn new(board: &'a Board, rotation: u8) -> Self {
        BoardView {
            board,
            rotation: rotation % 4,
        }
    }

    pub fn size(&self) -> u8 {
        self.board.size
    }

    pub fn get(&self, view_row: u8, view_col: u8) -> Option<Color> {
        let size = self.board.size - 1;
        let (board_row, board_col) = match self.rotation {
            0 => (view_row, view_col),
            1 => (view_col, size - view_row),
            2 => (size - view_row, size - view_col),
            3 => (size - view_col, view_row),
            _ => (view_row, view_col),
        };
        self.board.get(board_row, board_col)
    }
}
```

**Step 4: Export module in lib.rs**

Add to `src/lib.rs`:

```rust
pub mod board_view;
```

**Step 5: Run test to verify it passes**

Run: `cargo test test_rotation_0_degrees`

Expected: PASS

**Step 6: Commit**

```bash
git add src/board_view.rs src/lib.rs tests/board_view_tests.rs
git commit -m "feat: add BoardView with 0° rotation support"
```

---

## Task 2: Add 180° Rotation Test and Verify

**Files:**
- Test: `tests/board_view_tests.rs`

**Step 1: Write test for 180° rotation**

Add to `tests/board_view_tests.rs`:

```rust
#[test]
fn test_rotation_180_degrees() {
    let mut board = Board::new(19);
    board.set(0, 0, Color::Black);
    board.set(5, 10, Color::White);

    let view = BoardView::new(&board, 2);

    // Stone at (0,0) should appear at (18,18) in rotated view
    assert_eq!(view.get(18, 18), Some(Color::Black));
    // Stone at (5,10) should appear at (13,8) in rotated view
    assert_eq!(view.get(13, 8), Some(Color::White));
}
```

**Step 2: Run test to verify it passes**

Run: `cargo test test_rotation_180_degrees`

Expected: PASS (already implemented in BoardView::get)

**Step 3: Add test for rotation normalization**

Add to `tests/board_view_tests.rs`:

```rust
#[test]
fn test_rotation_normalization() {
    let mut board = Board::new(19);
    board.set(0, 0, Color::Black);

    // rotation=4 should behave like rotation=0
    let view4 = BoardView::new(&board, 4);
    assert_eq!(view4.get(0, 0), Some(Color::Black));

    // rotation=5 should behave like rotation=1
    let view5 = BoardView::new(&board, 5);
    assert_eq!(view5.get(18, 0), Some(Color::Black));
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_rotation_normalization`

Expected: PASS

**Step 5: Commit**

```bash
git add tests/board_view_tests.rs
git commit -m "test: add comprehensive BoardView rotation tests"
```

---

## Task 3: Add Rotation State to GameState

**Files:**
- Modify: `src/game.rs`
- Test: `tests/game_tests.rs`

**Step 1: Write failing test for rotation on loop**

Add to `tests/game_tests.rs`:

```rust
#[test]
fn test_board_rotates_on_loop() {
    let moves = vec![
        Move { color: Color::Black, position: Some((3, 3)) },
        Move { color: Color::White, position: Some((3, 4)) },
    ];
    let mut game = GameState::new(19, moves);

    assert_eq!(game.rotation(), 0);

    // Advance to end and loop
    game.next(); // move 1
    game.next(); // move 2
    game.next(); // should loop back and rotate

    assert_eq!(game.current_move, 0);
    assert_eq!(game.rotation(), 2); // 180 degrees
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_board_rotates_on_loop`

Expected: FAIL "no method named `rotation`"

**Step 3: Add rotation field to GameState**

In `src/game.rs`, modify GameState struct:

```rust
pub struct GameState {
    pub board: Board,
    pub moves: Vec<Move>,
    pub current_move: usize,
    pub properties: HashMap<String, Vec<String>>,
    looping_enabled: bool,
    rotation: u8,
}
```

**Step 4: Update constructors**

In `src/game.rs`, update `new()`:

```rust
pub fn new(board_size: u8, moves: Vec<Move>) -> Self {
    GameState {
        board: Board::new(board_size),
        moves,
        current_move: 0,
        properties: HashMap::new(),
        looping_enabled: true,
        rotation: 0,
    }
}
```

Update `with_properties()`:

```rust
pub fn with_properties(
    board_size: u8,
    moves: Vec<Move>,
    properties: HashMap<String, Vec<String>>,
) -> Self {
    GameState {
        board: Board::new(board_size),
        moves,
        current_move: 0,
        properties,
        looping_enabled: true,
        rotation: 0,
    }
}
```

**Step 5: Add rotation getter**

In `src/game.rs`:

```rust
impl GameState {
    // ... existing methods ...

    pub fn rotation(&self) -> u8 {
        self.rotation
    }
}
```

**Step 6: Run test to verify it still fails**

Run: `cargo test test_board_rotates_on_loop`

Expected: FAIL "assertion failed: rotation() == 2" (still returns 0)

**Step 7: Implement rotation on loop**

In `src/game.rs`, modify the `next()` method:

```rust
pub fn next(&mut self) -> bool {
    if self.current_move >= self.moves.len() {
        // At the end of the game
        if self.looping_enabled {
            // Loop back to the beginning with 180° rotation
            self.rotation = (self.rotation + 2) % 4;
            self.jump_to_start();
            return true;
        } else {
            // Don't loop, stay at end
            return false;
        }
    }

    // Apply the move at current_move index
    if let Some(pos) = self.moves[self.current_move].position {
        self.board
            .set(pos.0, pos.1, self.moves[self.current_move].color.clone());
    }

    self.current_move += 1;
    true
}
```

**Step 8: Run test to verify it passes**

Run: `cargo test test_board_rotates_on_loop`

Expected: PASS

**Step 9: Commit**

```bash
git add src/game.rs tests/game_tests.rs
git commit -m "feat: add rotation state that increments on loop"
```

---

## Task 4: Test Multiple Loop Rotations

**Files:**
- Test: `tests/game_tests.rs`

**Step 1: Write test for multiple loops**

Add to `tests/game_tests.rs`:

```rust
#[test]
fn test_multiple_loop_rotations() {
    let moves = vec![
        Move { color: Color::Black, position: Some((3, 3)) },
    ];
    let mut game = GameState::new(19, moves);

    assert_eq!(game.rotation(), 0);

    // First loop
    game.next(); // move 1
    game.next(); // loop back
    assert_eq!(game.rotation(), 2); // 180°

    // Second loop
    game.next(); // move 1
    game.next(); // loop back
    assert_eq!(game.rotation(), 0); // back to 0° (2+2=4, 4%4=0)

    // Third loop
    game.next(); // move 1
    game.next(); // loop back
    assert_eq!(game.rotation(), 2); // 180° again
}
```

**Step 2: Run test to verify it passes**

Run: `cargo test test_multiple_loop_rotations`

Expected: PASS

**Step 3: Test that disabled looping doesn't rotate**

Add to `tests/game_tests.rs`:

```rust
#[test]
fn test_no_rotation_when_looping_disabled() {
    let moves = vec![
        Move { color: Color::Black, position: Some((3, 3)) },
    ];
    let mut game = GameState::new(19, moves);
    game.set_looping(false);

    assert_eq!(game.rotation(), 0);

    game.next(); // move 1
    let continued = game.next(); // should NOT loop

    assert_eq!(continued, false);
    assert_eq!(game.rotation(), 0); // still 0°, no rotation
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_no_rotation_when_looping_disabled`

Expected: PASS

**Step 5: Commit**

```bash
git add tests/game_tests.rs
git commit -m "test: verify multiple loop rotations and no-loop behavior"
```

---

## Task 5: Update UI to Use BoardView

**Files:**
- Modify: `src/ui.rs`

**Step 1: Update render_board signature and create BoardView**

In `src/ui.rs`, find `render_board` function and modify:

```rust
use crate::board_view::BoardView;

fn render_board(frame: &mut Frame, area: Rect, board: &Board, rotation: u8) {
    let board_view = BoardView::new(board, rotation);
    let size = board_view.size() as usize;
    let mut lines = Vec::new();

    for row in 0..size {
        let mut spans = Vec::new();

        for col in 0..size {
            match board_view.get(row as u8, col as u8) {
                Some(Color::Black) => {
                    // Black stone emoji - naturally takes 2 char widths
                    spans.push(Span::styled("⚫", Style::default()));
                }
                Some(Color::White) => {
                    // White stone emoji - naturally takes 2 char widths
                    spans.push(Span::styled("⚪", Style::default()));
                }
                None => {
                    // Draw intersection in dark gray (toned down)
                    // Use 2 characters for each intersection to match emoji width
                    let intersection = if row == 0 && col == 0 {
                        "┌─"
                    } else if row == 0 && col == size - 1 {
                        "─┐"
                    } else if row == size - 1 && col == 0 {
                        "└─"
                    } else if row == size - 1 && col == size - 1 {
                        "─┘"
                    } else if row == 0 {
                        "─┬"
                    } else if row == size - 1 {
                        "─┴"
                    } else if col == 0 {
                        "├─"
                    } else if col == size - 1 {
                        "─┤"
                    } else {
                        "─┼"
                    };
                    spans.push(Span::styled(
                        intersection,
                        Style::default().fg(RatatuiColor::DarkGray),
                    ));
                }
            }

            // Add horizontal line between intersections (except last column)
            if col < size - 1 {
                spans.push(Span::styled(
                    "─",
                    Style::default().fg(RatatuiColor::DarkGray),
                ));
            }
        }

        lines.push(Line::from(spans));
    }

    // Rest of the function (padding, wood background) stays the same
    use ratatui::layout::Margin;
    let max_board_width = 65;
    let max_board_height = 27;

    let horizontal_padding = if area.width > max_board_width {
        (area.width - max_board_width) / 2
    } else {
        2
    };

    let vertical_padding = if area.height > max_board_height {
        (area.height - max_board_height) / 2
    } else {
        1
    };

    let board_area = area.inner(Margin {
        horizontal: horizontal_padding,
        vertical: vertical_padding,
    });

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default().style(Style::default().bg(RatatuiColor::Rgb(210, 180, 140))),
    );

    frame.render_widget(paragraph, board_area);
}
```

**Step 2: Update render_game to pass rotation**

In `src/ui.rs`, update `render_game`:

```rust
pub fn render_game(frame: &mut Frame, game: &GameState, auto_play: bool, playback_speed: u64) {
    // Calculate exact board height (19 lines for 19x19 board)
    let board_height = game.board.size as u16;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),                // Header
            Constraint::Length(board_height + 2), // Board + padding
            Constraint::Min(0),                   // Fill remaining space
            Constraint::Length(3),                // Status
        ])
        .split(frame.area());

    render_header(frame, chunks[0], game);
    render_board(frame, chunks[1], &game.board, game.rotation());
    render_status(frame, chunks[3], game, auto_play, playback_speed);
}
```

**Step 3: Build to verify compilation**

Run: `cargo build`

Expected: Success with no errors

**Step 4: Run all tests**

Run: `cargo test`

Expected: All tests PASS

**Step 5: Commit**

```bash
git add src/ui.rs
git commit -m "feat: integrate BoardView into rendering pipeline"
```

---

## Task 6: Manual Testing and Final Verification

**Files:**
- N/A (manual testing)

**Step 1: Build release binary**

Run: `cargo build --release`

Expected: Success

**Step 2: Test with sample SGF file**

Run: `./target/release/smartgameviewer sgf/<any-sgf-file>`

**Manual test checklist:**
- [ ] Game starts playing automatically
- [ ] Let game play to the end and loop back
- [ ] Verify board rotates 180° on loop (top-left stone now at bottom-right)
- [ ] Let it loop again, verify board returns to original orientation
- [ ] Press 'L' to disable looping
- [ ] Advance to end manually with arrow keys
- [ ] Verify board does NOT rotate when looping is disabled
- [ ] Press 'L' to re-enable looping
- [ ] Let it loop and verify rotation happens again
- [ ] Test at different speeds (1x, 2x, 3x) - rotation should work at all speeds

**Step 3: Run full test suite one more time**

Run: `cargo test`

Expected: All 15+ tests PASS

**Step 4: Final commit**

```bash
git commit --allow-empty -m "test: manual verification complete"
```

---

## Rollback to Approach A (If Needed)

If BoardView approach proves problematic, here's the simple fallback:

1. Delete `src/board_view.rs` and `tests/board_view_tests.rs`
2. Remove `pub mod board_view;` from `src/lib.rs`
3. In `src/ui.rs`, inline the transformation directly in `render_board`:

```rust
fn render_board(frame: &mut Frame, area: Rect, board: &Board, rotation: u8) {
    let size = board.size as usize;
    let max_coord = (board.size - 1) as u8;

    for row in 0..size {
        for col in 0..size {
            // Transform coordinates based on rotation
            let (board_row, board_col) = match rotation % 4 {
                0 => (row as u8, col as u8),
                2 => (max_coord - row as u8, max_coord - col as u8),
                _ => (row as u8, col as u8), // Only support 0° and 180° for now
            };

            match board.get(board_row, board_col) {
                // ... rest of rendering
            }
        }
    }
}
```

This keeps rotation in GameState but simplifies the transformation logic.

---

## Summary

**Total tasks:** 6 tasks with ~26 steps
**Estimated time:** 30-45 minutes
**Key principles:** TDD throughout, frequent commits, clean abstractions
**Testing:** Unit tests for BoardView, integration tests for GameState rotation, manual testing for UI

The plan uses Test-Driven Development at each step and maintains clean separation of concerns with BoardView handling all transformation logic.
