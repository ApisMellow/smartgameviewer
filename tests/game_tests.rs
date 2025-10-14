use smartgameviewer::game::*;
use smartgameviewer::parser::*;

#[test]
fn test_empty_board() {
    let board = Board::new(19);
    assert_eq!(board.size, 19);
    assert_eq!(board.get(0, 0), None);
    assert_eq!(board.get(18, 18), None);
}

#[test]
fn test_game_state_navigation() {
    let moves = vec![
        Move {
            color: Color::Black,
            position: Some((3, 3)), // row 3, col 3
            comment: None,
        },
        Move {
            color: Color::White,
            position: Some((3, 15)), // row 3, col 15
            comment: None,
        },
        Move {
            color: Color::Black,
            position: Some((15, 3)), // row 15, col 3
            comment: None,
        },
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
    assert_eq!(game.board.get(3, 15), Some(Color::White));

    // Move back
    game.previous();
    assert_eq!(game.current_move, 1);
    assert_eq!(game.board.get(3, 3), Some(Color::Black));
    assert_eq!(game.board.get(3, 15), None);
}

#[test]
fn test_coordinate_system_correctness() {
    // Test that SGF coordinates are correctly converted to board positions
    // SGF format: first letter = column (horizontal), second letter = row (vertical)
    // Board indexing: grid[row][col]

    let sgf = "(;GM[1]SZ[19];B[aa];W[ba];B[ab];W[sa])";
    let parsed = parse_sgf(sgf).unwrap();

    let mut game = GameState::new(19, parsed.moves);

    // First move: 'aa' = col 0, row 0 -> should be at board position (0, 0)
    game.next();
    assert_eq!(game.board.get(0, 0), Some(Color::Black));

    // Second move: 'ba' = col 1, row 0 -> should be at board position (0, 1)
    game.next();
    assert_eq!(game.board.get(0, 1), Some(Color::White));

    // Third move: 'ab' = col 0, row 1 -> should be at board position (1, 0)
    game.next();
    assert_eq!(game.board.get(1, 0), Some(Color::Black));

    // Fourth move: 'sa' = col 18, row 0 -> should be at board position (0, 18)
    game.next();
    assert_eq!(game.board.get(0, 18), Some(Color::White));
}

#[test]
fn test_game_looping_enabled() {
    let moves = vec![
        Move {
            color: Color::Black,
            position: Some((3, 3)),
            comment: None,
        },
        Move {
            color: Color::White,
            position: Some((3, 15)),
            comment: None,
        },
    ];

    let mut game = GameState::new(19, moves);

    // Enable looping (default should be true)
    assert_eq!(game.is_looping_enabled(), true);

    // Navigate to the end
    game.next(); // Move 1
    assert_eq!(game.current_move, 1);
    game.next(); // Move 2
    assert_eq!(game.current_move, 2);

    // Now we're at the end - next() should loop back to start
    let result = game.next();
    assert_eq!(result, true); // Should succeed
    assert_eq!(game.current_move, 0); // Should be back at start
    assert_eq!(game.board.get(3, 3), None); // Board should be empty
    assert_eq!(game.board.get(3, 15), None);
}

#[test]
fn test_game_looping_disabled() {
    let moves = vec![
        Move {
            color: Color::Black,
            position: Some((3, 3)),
            comment: None,
        },
        Move {
            color: Color::White,
            position: Some((3, 15)),
            comment: None,
        },
    ];

    let mut game = GameState::new(19, moves);

    // Disable looping
    game.set_looping(false);
    assert_eq!(game.is_looping_enabled(), false);

    // Navigate to the end
    game.next(); // Move 1
    game.next(); // Move 2

    // Now we're at the end - next() should return false (no looping)
    let result = game.next();
    assert_eq!(result, false); // Should fail
    assert_eq!(game.current_move, 2); // Should stay at end
}

#[test]
fn test_toggle_looping() {
    let moves = vec![Move {
        color: Color::Black,
        position: Some((3, 3)),
        comment: None,
    }];

    let mut game = GameState::new(19, moves);

    // Should start with looping enabled (default)
    assert_eq!(game.is_looping_enabled(), true);

    // Toggle it off
    game.toggle_looping();
    assert_eq!(game.is_looping_enabled(), false);

    // Toggle it back on
    game.toggle_looping();
    assert_eq!(game.is_looping_enabled(), true);
}
