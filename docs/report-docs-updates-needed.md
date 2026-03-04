# Documentation Updates Needed

Generated: 2026-03-04

This report identifies inaccuracies, missing information, and stale content across all project documentation. Issues are grouped by file and severity.

---

## README.md

### Factual errors

**Missing keyboard controls.** The Controls section lists 5 keys but the app handles 7. Add:

| Key | Action | Source |
|-----|--------|--------|
| `L` | Toggle looping | `main.rs` line 229 |
| `S` | Cycle playback speed (1x → 2x → 3x → 1x) | `main.rs` lines 232–238 |

Both are also shown in the status bar hint: `"← → Step | Space Play/Pause | L Loop | S Speed | Q Quit"` (`ui.rs` line 316).

**Missing playlist/directory feature.** The Usage section only shows running with a single SGF file. The app now supports:
- `cargo run -- path/to/folder/` — plays all `.sgf` files in natural sort order
- `cargo run` (no args) — scans `./sgf/` by default

These modes should be documented with examples.

### Stale roadmap items

The Roadmap section lists these as future work, but they are already implemented:

| Roadmap item | Status | Where |
|--------------|--------|-------|
| `[ ] Colors, gradients, shine effects` | Partially done | Title shine (`ui.rs` lines 37–82), star animation (`ui.rs` lines 265–305) |
| `[ ] Board background coloring (wood texture)` | Done | Tan wood RGB(210, 180, 140) at `ui.rs` line 196 |

Update these to `[x]` or revise the descriptions to reflect what remains.

---

## TODO.md

### Stale items

| Item | Status | Notes |
|------|--------|-------|
| `[ ] Implement board background coloring (wood texture simulation)` | Already done | `ui.rs` line 196: `RatatuiColor::Rgb(210, 180, 140)` |

Mark as complete or remove.

---

## docs/manual-testing-verification.md

### Factual error

**Lines 74–76 — wrong speed keys:**

Current text says:
> Press '1' for 1x speed, '2' for 2x speed, '3' for 3x speed

Actual implementation (`main.rs` lines 232–238): the `S` key cycles through speeds 1x → 2x → 3x → 1x. There are no `1`/`2`/`3` key bindings.

**Fix:** Replace with: "Press 'S' to cycle playback speed (1x → 2x → 3x → 1x)"

### Stale content

| Issue | Location | Notes |
|-------|----------|-------|
| Binary path references deleted worktree | Line 6 | Path `.worktrees/board-rotation/target/...` no longer exists. Change to `target/release/smartgameviewer` |
| Test count says 21 | Body | Current count is 27 (6 playlist tests added since). Update or add a note that this was a point-in-time snapshot |

---

## docs/playlist-manual-testing.md

### Factual error

**Line 33 — missing `--` separator:**

Current text:
```
cargo run sgf/game1.sgf
```

Should be:
```
cargo run -- sgf/game1.sgf
```

Without `--`, the argument is passed to Cargo, not to the binary.

### Minor inaccuracy

**"Invalid SGF file → skips it gracefully"** — the checklist implies the app skips bad files and continues. Actual behavior (`main.rs` lines 160–165): it prints an error with `eprintln!` and stops auto-play. It does not skip to the next file. Reword to match actual behavior, e.g.: "Invalid SGF file → stops auto-play and shows error."

---

## docs/code-walkthrough.md

### Minor inaccuracy

**Star animation frame breakdown** — the description groups frames imprecisely:

Current text says:
> - Frames 4–5: bright yellow asterisks
> - Frames 5–7: peak brightness bold stars

Frame 5 appears in both ranges. The actual frame mapping in `ui.rs`:

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

Replace the current frame groupings with this table or correct the ranges to not overlap.

### Everything else is accurate

Module layout, data types, `AppState` enum, timing values, parser algorithm, `BoardView` rotation table, playlist API, natural sort description, and test counts (27) are all verified correct against the source code.

---

## docs/plans/ (2 files)

These are historical implementation plans. They contain intermediate code snippets that differ from the final implementation (e.g., `sgf_to_coords` originally returned `(col, row)` instead of `(row, col)`). This is expected for archived design documents.

**No updates needed** — these are historical artifacts, not living documentation.

---

## Summary

| File | Severity | Issue | Action |
|------|----------|-------|--------|
| `README.md` | High | Missing `L` and `S` key controls | Add to Controls section |
| `README.md` | High | No mention of directory/playlist mode | Add usage examples |
| `README.md` | Low | Roadmap items already done | Mark completed |
| `TODO.md` | Low | Board background listed as TODO | Mark completed |
| `manual-testing-verification.md` | High | Speed keys listed as `1`/`2`/`3` | Change to `S` cycles |
| `manual-testing-verification.md` | Low | Worktree path and test count stale | Update or annotate |
| `playlist-manual-testing.md` | Medium | Missing `--` in cargo run command | Add `--` separator |
| `playlist-manual-testing.md` | Low | "Skips gracefully" overstates behavior | Reword |
| `code-walkthrough.md` | Low | Star animation frame overlap | Fix frame ranges |
