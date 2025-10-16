# Playlist Feature Manual Testing Checklist

## Setup
- [ ] Verify ./sgf folder exists with at least 3 SGF files
- [ ] Name test files: game1.sgf, game2.sgf, game10.sgf

## Test Cases

### Multi-File Playback
- [ ] Run with no args: `cargo run`
- [ ] Verify it loads ./sgf folder automatically
- [ ] Verify files play in natural sort order (1, 2, 10)
- [ ] Verify 3-second transition appears between files
- [ ] Verify transition shows "Next: [Game Title]"
- [ ] Verify transition text pulses orange/yellow
- [ ] Press Q during transition → quits immediately

### Enhanced Star Animation
- [ ] Verify star animates with 10 frames during playback
- [ ] Verify star shows color fade from gray → yellow → white
- [ ] Verify star stops during transition
- [ ] Verify star stops when paused (Space key)
- [ ] Verify star animation speed matches playback speed

### Looping Behavior
- [ ] With looping ON: reaches last file → transitions back to first
- [ ] With looping OFF: reaches last file → stops on final board
- [ ] Verify stopped state: star static, can still navigate, Q to quit

### Single File Mode (Backwards Compatibility)
- [ ] Run with file arg: `cargo run sgf/game1.sgf`
- [ ] Verify works exactly as before (no transitions)
- [ ] Verify looping works on single file

### Series Grouping
- [ ] Create series: AlphaGo_game1.sgf, AlphaGo_game2.sgf, AlphaGo_game10.sgf
- [ ] Verify natural sort keeps them in order (1, 2, 10)

### Speed Control
- [ ] Press S to cycle speeds during playback
- [ ] Verify star animation speed changes with playback speed
- [ ] Verify transition duration stays 3 seconds regardless

### Edge Cases
- [ ] Empty ./sgf folder → clear error message
- [ ] Invalid SGF file → skips it gracefully
- [ ] Press L to toggle looping during multi-file playback
