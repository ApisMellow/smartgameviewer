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
