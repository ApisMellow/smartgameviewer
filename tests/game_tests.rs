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

#[test]
fn test_board_rotates_on_loop() {
    let moves = vec![
        Move {
            color: Color::Black,
            position: Some((3, 3)),
            comment: None,
        },
        Move {
            color: Color::White,
            position: Some((3, 4)),
            comment: None,
        },
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

#[test]
fn test_multiple_loop_rotations() {
    let moves = vec![Move {
        color: Color::Black,
        position: Some((3, 3)),
        comment: None,
    }];
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

#[test]
fn test_no_rotation_when_looping_disabled() {
    let moves = vec![Move {
        color: Color::Black,
        position: Some((3, 3)),
        comment: None,
    }];
    let mut game = GameState::new(19, moves);
    game.set_looping(false);

    assert_eq!(game.rotation(), 0);

    game.next(); // move 1
    let continued = game.next(); // should NOT loop

    assert_eq!(continued, false);
    assert_eq!(game.rotation(), 0); // still 0°, no rotation
}

#[test]
fn test_with_properties_and_get_property() {
    let mut props = std::collections::HashMap::new();
    props.insert("PB".to_string(), vec!["Alice".to_string()]);
    props.insert("PW".to_string(), vec!["Bob".to_string()]);
    props.insert("GN".to_string(), vec!["My Game".to_string()]);

    let game = GameState::with_properties(19, vec![], props);

    assert_eq!(game.get_property("PB"), Some("Alice"));
    assert_eq!(game.get_property("PW"), Some("Bob"));
    assert_eq!(game.get_property("GN"), Some("My Game"));
    assert_eq!(game.get_property("MISSING"), None);
}

#[test]
fn test_previous_at_move_zero() {
    let moves = vec![Move {
        color: Color::Black,
        position: Some((3, 3)),
        comment: None,
    }];
    let mut game = GameState::new(19, moves);

    // At move 0, previous should return false and board stays empty
    let result = game.previous();
    assert_eq!(result, false);
    assert_eq!(game.current_move, 0);
    assert_eq!(game.board.get(3, 3), None);
}

#[test]
fn test_next_with_pass_move() {
    let moves = vec![
        Move {
            color: Color::Black,
            position: None, // pass
            comment: None,
        },
        Move {
            color: Color::White,
            position: Some((3, 3)),
            comment: None,
        },
    ];
    let mut game = GameState::new(19, moves);

    // Pass move: current_move increments but no stone placed
    game.next();
    assert_eq!(game.current_move, 1);
    // Board should still be empty - pass places no stone
    for r in 0..19u8 {
        for c in 0..19u8 {
            assert_eq!(game.board.get(r, c), None);
        }
    }

    // Next move places a stone
    game.next();
    assert_eq!(game.current_move, 2);
    assert_eq!(game.board.get(3, 3), Some(Color::White));
}

#[test]
fn test_jump_to_end_with_zero_moves() {
    let game_moves: Vec<Move> = vec![];
    let mut game = GameState::new(19, game_moves);

    game.jump_to_end();
    assert_eq!(game.current_move, 0);
    // Board should still be empty
    assert_eq!(game.board.get(0, 0), None);
}

#[test]
fn test_jump_to_start_when_already_at_start() {
    let moves = vec![Move {
        color: Color::Black,
        position: Some((3, 3)),
        comment: None,
    }];
    let mut game = GameState::new(19, moves);

    // Already at start - should be idempotent
    game.jump_to_start();
    assert_eq!(game.current_move, 0);
    assert_eq!(game.board.get(3, 3), None);
}

#[test]
fn test_board_clear() {
    let mut board = Board::new(19);
    board.set(5, 5, Color::Black);
    assert_eq!(board.get(5, 5), Some(Color::Black));

    board.clear(5, 5);
    assert_eq!(board.get(5, 5), None);
}

#[test]
fn test_non_19x19_board() {
    let moves = vec![
        Move {
            color: Color::Black,
            position: Some((0, 0)),
            comment: None,
        },
        Move {
            color: Color::White,
            position: Some((8, 8)),
            comment: None,
        },
    ];
    let mut game = GameState::new(9, moves);

    assert_eq!(game.board.size, 9);

    game.next();
    assert_eq!(game.board.get(0, 0), Some(Color::Black));

    game.next();
    assert_eq!(game.board.get(8, 8), Some(Color::White));

    game.previous();
    assert_eq!(game.board.get(8, 8), None);
    assert_eq!(game.board.get(0, 0), Some(Color::Black));
}
