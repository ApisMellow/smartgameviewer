use smartgameviewer::parser::*;

#[test]
fn test_parse_empty_game() {
    let sgf = "(;)";
    let result = parse_sgf(sgf);
    assert!(result.is_ok());
    let game = result.unwrap();
    assert_eq!(game.moves.len(), 0);
}
