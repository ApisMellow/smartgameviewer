use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone)]
pub struct Move {
    pub color: Color,
    pub position: Option<(u8, u8)>, // None for pass
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct GameTree {
    pub properties: HashMap<String, Vec<String>>,
    pub moves: Vec<Move>,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat(String),
}

pub fn parse_sgf(_input: &str) -> Result<GameTree, ParseError> {
    Ok(GameTree {
        properties: HashMap::new(),
        moves: Vec::new(),
    })
}
