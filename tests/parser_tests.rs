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
