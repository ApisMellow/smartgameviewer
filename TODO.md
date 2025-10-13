# Smart Game Viewer - Future Enhancements

## Visual Polish

### Colors & Styling
- [ ] Add color gradients to black and white stones
- [ ] Implement board background coloring (wood texture simulation)
- [ ] Add colored borders for the active/last move
- [ ] Theme support (light/dark modes)

### Shine Effects
- [ ] Claude Code-style animated shine effect across the board
- [ ] Glossy stone appearance with lighting effects
- [ ] Smooth transitions when stones appear/disappear
- [ ] Highlight animation for captured stones

### Board Enhancements
- [ ] Display star points (hoshi) at traditional positions (4-4, etc.)
- [ ] Coordinate labels (A-T for columns, 1-19 for rows)
- [ ] Show captured stone count for each player
- [ ] Display komi and handicap information

## Sound Support

### Stone Placement Sounds
- [ ] Realistic stone placement sound effect (that satisfying "clack")
- [ ] Different sounds for black vs white stones
- [ ] Volume control
- [ ] Sound toggle on/off

### Additional Audio
- [ ] Ambient background music option
- [ ] Sound for passing
- [ ] Audio cues for game start/end
- [ ] Optional timer sound effects

## Functionality

### Advanced Navigation
- [ ] Variation support (tree navigation for games with branches)
- [ ] Jump to move number (e.g., "goto move 50")
- [ ] Search comments for keywords
- [ ] Bookmark important positions

### Analysis Features
- [ ] Show territory counting (Chinese/Japanese rules)
- [ ] Display captured stones with animation
- [ ] Highlight illegal moves
- [ ] Show ko situations
- [ ] Life and death status indicators

### File Management
- [ ] Recent files list
- [ ] File browser/picker in TUI
- [ ] Drag and drop SGF files (when in GUI)
- [ ] Save board position as image

## AI Integration

### Game Analysis
- [ ] Connect to Katago or Leela Zero for move analysis
- [ ] Show win rate graph
- [ ] Display top move suggestions
- [ ] Heat map of good moves
- [ ] Variation trees with AI commentary

### Playing Features
- [ ] Play against AI
- [ ] Get hints during game review
- [ ] Problem/puzzle mode with AI verification

## Platform Expansion

### GUI Version
- [ ] Build with Tauri for native desktop app
- [ ] Or use egui for cross-platform GUI
- [ ] Mouse support for navigation
- [ ] Drag board to resize/move
- [ ] Menu bar with File/Edit/View options

### Web Version
- [ ] Compile to WASM for web deployment
- [ ] Host on GitHub Pages or similar
- [ ] Share game URLs
- [ ] Embed in websites

### Mobile
- [ ] Touch-friendly interface
- [ ] Swipe gestures for navigation
- [ ] Pinch to zoom
- [ ] iOS/Android apps (via Tauri Mobile)

## Performance

### Optimization
- [ ] Lazy loading for very large SGF files
- [ ] Efficient undo stack instead of board rebuilding
- [ ] Cache rendered board states
- [ ] Multi-threaded parsing for large collections

### Testing
- [ ] Fuzzing for SGF parser
- [ ] Performance benchmarks
- [ ] Test with SGF files >10MB
- [ ] Memory profiling

## User Experience

### Configuration
- [ ] Config file for user preferences
- [ ] Customizable key bindings
- [ ] Adjustable auto-play speed
- [ ] Font size options

### Accessibility
- [ ] Screen reader support
- [ ] High contrast mode
- [ ] Keyboard-only navigation (already done!)
- [ ] Colorblind-friendly stone colors

### Documentation
- [ ] Video demo/tutorial
- [ ] SGF format documentation
- [ ] Contributing guide
- [ ] Architecture documentation

## Distribution

### Packaging
- [ ] Homebrew formula for macOS
- [ ] AUR package for Arch Linux
- [ ] Cargo publish to crates.io
- [ ] Snap/Flatpak for Linux
- [ ] Windows installer

### Community
- [ ] Set up discussions on GitHub
- [ ] Create example gallery
- [ ] User-contributed themes
- [ ] SGF file collection

## Nice to Have

- [ ] Tournament mode (navigate through multiple games)
- [ ] Two-player hot-seat mode
- [ ] SGF editing capabilities
- [ ] Import from online Go servers (OGS, KGS, etc.)
- [ ] Live game streaming support
- [ ] Joseki dictionary integration
- [ ] Opening book/database

---

**Priority Order:** Visual Polish → Sound → GUI → AI Integration → Web Version

**Current Version:** v0.1.0 (Terminal viewer with basic functionality)
