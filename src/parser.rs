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

pub fn parse_sgf(input: &str) -> Result<GameTree, ParseError> {
    let input = input.trim();

    if !input.starts_with('(') || !input.ends_with(')') {
        return Err(ParseError::InvalidFormat(
            "Missing outer parentheses".to_string(),
        ));
    }

    // Remove outer parentheses
    let content = &input[1..input.len() - 1];

    if !content.starts_with(';') {
        return Err(ParseError::InvalidFormat(
            "Missing initial semicolon".to_string(),
        ));
    }

    let mut properties = HashMap::new();
    let moves = Vec::new();

    // Simple property parser - finds KEY[value] patterns
    let mut chars = content.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_uppercase() {
            // Found a property key
            let mut key = String::new();
            key.push(ch);

            // Read rest of key
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_ascii_uppercase() {
                    key.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            // Read value(s) in brackets
            let mut values = Vec::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '[' {
                    chars.next(); // consume '['
                    let mut value = String::new();

                    while let Some(val_ch) = chars.next() {
                        if val_ch == ']' {
                            break;
                        }
                        value.push(val_ch);
                    }

                    values.push(value);
                } else {
                    break;
                }
            }

            if !values.is_empty() {
                properties.insert(key, values);
            }
        }
    }

    Ok(GameTree { properties, moves })
}
