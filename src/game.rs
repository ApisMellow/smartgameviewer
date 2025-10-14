use crate::parser::{Color, Move};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Board {
    pub size: u8,
    grid: Vec<Vec<Option<Color>>>,
}

impl Board {
    pub fn new(size: u8) -> Self {
        Board {
            size,
            grid: vec![vec![None; size as usize]; size as usize],
        }
    }

    pub fn get(&self, row: u8, col: u8) -> Option<Color> {
        self.grid[row as usize][col as usize].clone()
    }

    pub fn set(&mut self, row: u8, col: u8, color: Color) {
        self.grid[row as usize][col as usize] = Some(color);
    }

    pub fn clear(&mut self, row: u8, col: u8) {
        self.grid[row as usize][col as usize] = None;
    }
}

pub struct GameState {
    pub board: Board,
    pub moves: Vec<Move>,
    pub current_move: usize, // 0 = empty board, 1 = after first move, etc.
    pub properties: HashMap<String, Vec<String>>, // Game metadata
    looping_enabled: bool, // Whether to loop back to start when reaching the end
}

impl GameState {
    pub fn new(board_size: u8, moves: Vec<Move>) -> Self {
        GameState {
            board: Board::new(board_size),
            moves,
            current_move: 0,
            properties: HashMap::new(),
            looping_enabled: true, // Default to looping enabled
        }
    }

    pub fn with_properties(
        board_size: u8,
        moves: Vec<Move>,
        properties: HashMap<String, Vec<String>>,
    ) -> Self {
        GameState {
            board: Board::new(board_size),
            moves,
            current_move: 0,
            properties,
            looping_enabled: true, // Default to looping enabled
        }
    }

    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties
            .get(key)
            .and_then(|v| v.first())
            .map(|s| s.as_str())
    }

    pub fn is_looping_enabled(&self) -> bool {
        self.looping_enabled
    }

    pub fn set_looping(&mut self, enabled: bool) {
        self.looping_enabled = enabled;
    }

    pub fn toggle_looping(&mut self) {
        self.looping_enabled = !self.looping_enabled;
    }

    pub fn next(&mut self) -> bool {
        if self.current_move >= self.moves.len() {
            // At the end of the game
            if self.looping_enabled {
                // Loop back to the beginning
                self.jump_to_start();
                return true;
            } else {
                // Don't loop, stay at end
                return false;
            }
        }

        // Apply the move at current_move index
        if let Some(pos) = self.moves[self.current_move].position {
            self.board
                .set(pos.0, pos.1, self.moves[self.current_move].color.clone());
        }

        self.current_move += 1;
        true
    }

    pub fn previous(&mut self) -> bool {
        if self.current_move == 0 {
            return false; // Already at start
        }

        self.current_move -= 1;

        // Rebuild board from scratch up to current position
        self.board = Board::new(self.board.size);
        for i in 0..self.current_move {
            if let Some(pos) = self.moves[i].position {
                self.board.set(pos.0, pos.1, self.moves[i].color.clone());
            }
        }

        true
    }

    pub fn jump_to_start(&mut self) {
        self.current_move = 0;
        self.board = Board::new(self.board.size);
    }

    pub fn jump_to_end(&mut self) {
        // Jump directly to the end without triggering looping behavior
        self.current_move = 0;
        self.board = Board::new(self.board.size);
        for i in 0..self.moves.len() {
            if let Some(pos) = self.moves[i].position {
                self.board.set(pos.0, pos.1, self.moves[i].color.clone());
            }
        }
        self.current_move = self.moves.len();
    }
}
