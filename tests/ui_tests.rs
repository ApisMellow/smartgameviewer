use smartgameviewer::{
    game::{Board, GameState},
    parser::{Color, Move},
};

#[test]
fn test_board_state_after_moves() {
    // Create a simple game with a few moves
    let moves = vec![
        Move {
            color: Color::Black,
            position: Some((3, 3)),
            comment: None,
        },
        Move {
            color: Color::White,
            position: Some((15, 3)),
            comment: None,
        },
        Move {
            color: Color::Black,
            position: Some((3, 15)),
            comment: None,
        },
    ];

    let mut game_state = GameState::new(19, moves);

    // Initially board should be empty
    assert_eq!(game_state.board.get(3, 3), None);
    assert_eq!(game_state.board.get(15, 3), None);
    assert_eq!(game_state.board.get(3, 15), None);

    // After first move, only one stone should be on board
    game_state.next();
    assert_eq!(game_state.board.get(3, 3), Some(Color::Black));
    assert_eq!(game_state.board.get(15, 3), None);
    assert_eq!(game_state.board.get(3, 15), None);

    // After second move, two stones should be on board
    game_state.next();
    assert_eq!(game_state.board.get(3, 3), Some(Color::Black));
    assert_eq!(game_state.board.get(15, 3), Some(Color::White));
    assert_eq!(game_state.board.get(3, 15), None);

    // After third move, all three stones should be on board
    game_state.next();
    assert_eq!(game_state.board.get(3, 3), Some(Color::Black));
    assert_eq!(game_state.board.get(15, 3), Some(Color::White));
    assert_eq!(game_state.board.get(3, 15), Some(Color::Black));

    // Test going backward
    game_state.previous();
    assert_eq!(game_state.board.get(3, 3), Some(Color::Black));
    assert_eq!(game_state.board.get(15, 3), Some(Color::White));
    assert_eq!(game_state.board.get(3, 15), None);

    game_state.previous();
    assert_eq!(game_state.board.get(3, 3), Some(Color::Black));
    assert_eq!(game_state.board.get(15, 3), None);
    assert_eq!(game_state.board.get(3, 15), None);

    game_state.previous();
    assert_eq!(game_state.board.get(3, 3), None);
    assert_eq!(game_state.board.get(15, 3), None);
    assert_eq!(game_state.board.get(3, 15), None);
}

#[test]
fn test_empty_board_rendering_logic() {
    let board = Board::new(19);

    // Verify corner positions
    for row in 0..19 {
        for col in 0..19 {
            assert_eq!(
                board.get(row, col),
                None,
                "Position ({}, {}) should be empty",
                row,
                col
            );
        }
    }
}

#[test]
fn test_board_with_stones() {
    let mut board = Board::new(19);

    // Place some stones
    board.set(0, 0, Color::Black);
    board.set(18, 18, Color::White);
    board.set(9, 9, Color::Black);

    // Verify stones are placed correctly
    assert_eq!(board.get(0, 0), Some(Color::Black));
    assert_eq!(board.get(18, 18), Some(Color::White));
    assert_eq!(board.get(9, 9), Some(Color::Black));

    // Verify other positions are still empty
    assert_eq!(board.get(0, 1), None);
    assert_eq!(board.get(1, 0), None);
    assert_eq!(board.get(9, 10), None);
}
