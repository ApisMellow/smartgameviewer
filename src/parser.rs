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

// Convert SGF coordinates (e.g., "dd") to board position (3, 3)
// SGF uses 'a' = 0, 'b' = 1, etc.
fn sgf_to_coords(s: &str) -> Option<(u8, u8)> {
    if s.is_empty() {
        return None; // Pass move
    }

    let bytes = s.as_bytes();
    if bytes.len() != 2 {
        return None;
    }

    let col = bytes[0].wrapping_sub(b'a');
    let row = bytes[1].wrapping_sub(b'a');

    if col < 19 && row < 19 {
        Some((col, row))
    } else {
        None
    }
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
    let mut moves = Vec::new();

    // Split by semicolons to get nodes
    let nodes: Vec<&str> = content.split(';').filter(|s| !s.is_empty()).collect();

    for (idx, node) in nodes.iter().enumerate() {
        let mut chars = node.chars().peekable();

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
                    // Check if this is a move property
                    match key.as_str() {
                        "B" => {
                            moves.push(Move {
                                color: Color::Black,
                                position: sgf_to_coords(&values[0]),
                                comment: None,
                            });
                        }
                        "W" => {
                            moves.push(Move {
                                color: Color::White,
                                position: sgf_to_coords(&values[0]),
                                comment: None,
                            });
                        }
                        _ => {
                            // Store as property (only for first node - root properties)
                            if idx == 0 {
                                properties.insert(key, values);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(GameTree { properties, moves })
}
