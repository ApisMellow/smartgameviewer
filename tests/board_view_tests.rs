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

#[test]
fn test_rotation_90_degrees() {
    let mut board = Board::new(19);
    // Place stone at board (3, 5)
    board.set(3, 5, Color::Black);

    let view = BoardView::new(&board, 1);
    // 90° mapping: view(r,c) -> board(c, size-1-r)
    // To find view coords for board(3,5): c=3, size-1-r=5 -> r=13
    // So stone appears at view (13, 3)
    assert_eq!(view.get(13, 3), Some(Color::Black));
    // Original position should be empty in rotated view
    assert_eq!(view.get(3, 5), None);
}

#[test]
fn test_rotation_270_degrees() {
    let mut board = Board::new(19);
    // Place stone at board (3, 5)
    board.set(3, 5, Color::Black);

    let view = BoardView::new(&board, 3);
    // 270° mapping: view(r,c) -> board(size-1-c, r)
    // To find view coords for board(3,5): size-1-c=3 -> c=15, r=5
    // So stone appears at view (5, 15)
    assert_eq!(view.get(5, 15), Some(Color::Black));
    assert_eq!(view.get(3, 5), None);
}

#[test]
fn test_empty_cell_through_view() {
    let board = Board::new(19);
    let view = BoardView::new(&board, 2);

    // All cells should be None through any rotation
    assert_eq!(view.get(0, 0), None);
    assert_eq!(view.get(9, 9), None);
    assert_eq!(view.get(18, 18), None);
}

#[test]
fn test_board_view_non_19x19() {
    let mut board = Board::new(9);
    board.set(0, 0, Color::Black);
    board.set(8, 8, Color::White);

    // 0° - identity
    let view0 = BoardView::new(&board, 0);
    assert_eq!(view0.size(), 9);
    assert_eq!(view0.get(0, 0), Some(Color::Black));
    assert_eq!(view0.get(8, 8), Some(Color::White));

    // 180° on 9x9 board
    let view2 = BoardView::new(&board, 2);
    assert_eq!(view2.get(8, 8), Some(Color::Black)); // (0,0) -> (8,8)
    assert_eq!(view2.get(0, 0), Some(Color::White)); // (8,8) -> (0,0)
}
