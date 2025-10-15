use crate::game::Board;
use crate::parser::Color;

pub struct BoardView<'a> {
    board: &'a Board,
    rotation: u8,
}

impl<'a> BoardView<'a> {
    pub fn new(board: &'a Board, rotation: u8) -> Self {
        BoardView {
            board,
            rotation: rotation % 4,
        }
    }

    pub fn size(&self) -> u8 {
        self.board.size
    }

    pub fn get(&self, view_row: u8, view_col: u8) -> Option<Color> {
        let size = self.board.size - 1;
        let (board_row, board_col) = match self.rotation {
            0 => (view_row, view_col),
            1 => (view_col, size - view_row),
            2 => (size - view_row, size - view_col),
            3 => (size - view_col, view_row),
            _ => (view_row, view_col),
        };
        self.board.get(board_row, board_col)
    }
}
