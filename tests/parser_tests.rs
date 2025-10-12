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
    assert_eq!(game.moves[0].position, Some((3, 3))); // 'dd' = (3,3)
    assert_eq!(game.moves[1].color, Color::White);
    assert_eq!(game.moves[1].position, Some((15, 3))); // 'pd' = (15,3)
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
