use smartgameviewer::parser::*;

#[test]
fn test_parse_empty_game() {
    let sgf = "(;)";
    let result = parse_sgf(sgf);
    assert!(result.is_ok());
    let game = result.unwrap();
    assert_eq!(game.moves.len(), 0);
}

#[test]
fn test_parse_game_properties() {
    let sgf = "(;GM[1]FF[4]SZ[19]PB[Lee Sedol]PW[AlphaGo])";
    let result = parse_sgf(sgf);
    assert!(result.is_ok());
    let game = result.unwrap();
    assert_eq!(game.properties.get("SZ").unwrap()[0], "19");
    assert_eq!(game.properties.get("PB").unwrap()[0], "Lee Sedol");
    assert_eq!(game.properties.get("PW").unwrap()[0], "AlphaGo");
}

#[test]
fn test_parse_simple_moves() {
    let sgf = "(;GM[1]SZ[19];B[dd];W[pd];B[dp];W[pp])";
    let result = parse_sgf(sgf);
    assert!(result.is_ok());
    let game = result.unwrap();
    assert_eq!(game.moves.len(), 4);
    assert_eq!(game.moves[0].color, Color::Black);
    assert_eq!(game.moves[0].position, Some((3, 3))); // 'dd' = (row 3, col 3)
    assert_eq!(game.moves[1].color, Color::White);
    assert_eq!(game.moves[1].position, Some((3, 15))); // 'pd' = (row 3, col 15)
}

#[test]
fn test_parse_pass_move() {
    let sgf = "(;GM[1];B[];W[dd])";
    let result = parse_sgf(sgf);
    assert!(result.is_ok());
    let game = result.unwrap();
    assert_eq!(game.moves.len(), 2);
    assert_eq!(game.moves[0].position, None); // Pass
}

#[test]
fn test_parse_sgf_missing_parentheses() {
    assert!(parse_sgf("no parens").is_err());
    assert!(parse_sgf(";GM[1]").is_err());
    assert!(parse_sgf("(;GM[1]").is_err());
    assert!(parse_sgf(";GM[1])").is_err());
}

#[test]
fn test_parse_sgf_missing_semicolon() {
    assert!(parse_sgf("(no semicolon)").is_err());
    assert!(parse_sgf("(GM[1])").is_err());
}

#[test]
fn test_parse_sgf_empty_input() {
    assert!(parse_sgf("").is_err());
}

#[test]
fn test_parse_sgf_whitespace_handling() {
    // Leading/trailing whitespace should be trimmed
    let result = parse_sgf("  (;GM[1])  ");
    assert!(result.is_ok());
    let game = result.unwrap();
    assert_eq!(game.properties.get("GM").unwrap()[0], "1");
}

#[test]
fn test_parse_multi_value_properties() {
    // AB[dd][ee] should produce a Vec with 2 entries
    let sgf = "(;AB[dd][ee][ff])";
    let result = parse_sgf(sgf).unwrap();
    let ab_values = result.properties.get("AB").unwrap();
    assert_eq!(ab_values.len(), 3);
    assert_eq!(ab_values[0], "dd");
    assert_eq!(ab_values[1], "ee");
    assert_eq!(ab_values[2], "ff");
}

#[test]
fn test_non_root_node_properties_ignored() {
    // Properties on non-root nodes (other than B/W) should not be stored
    let sgf = "(;GM[1]SZ[19];B[dd]C[a comment];W[pp]C[another])";
    let result = parse_sgf(sgf).unwrap();
    // Root properties should be present
    assert!(result.properties.contains_key("GM"));
    assert!(result.properties.contains_key("SZ"));
    // C from non-root nodes should NOT be in properties
    assert!(!result.properties.contains_key("C"));
}
