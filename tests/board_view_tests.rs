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
