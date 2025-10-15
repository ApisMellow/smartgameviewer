use smartgameviewer::playlist::PlaylistManager;

#[test]
fn test_single_file_playlist() {
    let playlist = PlaylistManager::new(Some("sgf/AlphaGo_LeeSedol_game4.sgf")).unwrap();

    assert_eq!(playlist.is_single_file(), true);
    assert_eq!(playlist.has_next(), false);
    assert_eq!(
        playlist.current().to_str().unwrap(),
        "sgf/AlphaGo_LeeSedol_game4.sgf"
    );
}
