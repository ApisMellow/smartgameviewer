use smartgameviewer::{game::GameState, parser::parse_sgf};
use std::fs;

#[test]
fn test_load_alphago_game4() {
    let sgf_content = fs::read_to_string("examples/AlphaGo_LeeSedol_game4.sgf")
        .expect("Failed to read example SGF file");

    let game_tree = parse_sgf(&sgf_content).expect("Failed to parse SGF");

    // Verify board size
    let board_size = game_tree
        .properties
        .get("SZ")
        .and_then(|v| v.first())
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(19);
    assert_eq!(board_size, 19);

    // Verify we have moves
    assert!(game_tree.moves.len() > 0, "Game should have moves");

    // Verify player names
    assert_eq!(game_tree.properties.get("PB").unwrap()[0], "AlphaGo");
    assert_eq!(game_tree.properties.get("PW").unwrap()[0], "Lee Sedol");

    // Create game state
    let mut game_state = GameState::new(board_size, game_tree.moves.clone());

    // Test navigation
    assert_eq!(game_state.current_move, 0);

    // Move forward
    assert!(game_state.next());
    assert_eq!(game_state.current_move, 1);

    // Move backward
    assert!(game_state.previous());
    assert_eq!(game_state.current_move, 0);

    // Jump to end
    game_state.jump_to_end();
    assert_eq!(game_state.current_move, game_tree.moves.len());

    // Jump to start
    game_state.jump_to_start();
    assert_eq!(game_state.current_move, 0);
}

#[test]
fn test_load_alphago_game5() {
    let sgf_content = fs::read_to_string("examples/AlphaGo_LeeSedol_game5.sgf")
        .expect("Failed to read example SGF file");

    let game_tree = parse_sgf(&sgf_content).expect("Failed to parse SGF");

    // Verify we have moves
    assert!(game_tree.moves.len() > 0, "Game should have moves");

    let board_size = game_tree
        .properties
        .get("SZ")
        .and_then(|v| v.first())
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(19);

    let mut game_state = GameState::new(board_size, game_tree.moves);

    // Disable looping for this test
    game_state.set_looping(false);

    // Test that we can navigate through entire game
    let total_moves = game_state.moves.len();
    for _ in 0..total_moves {
        assert!(game_state.next());
    }
    assert_eq!(game_state.current_move, total_moves);

    // Verify we can't go past the end (with looping disabled)
    assert!(!game_state.next());
}
