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

#[test]
fn test_natural_sort_ordering() {
    // Create test files: game1, game10, game2, game20
    // They should sort as: game1, game2, game10, game20

    let mut playlist = PlaylistManager::new(Some("test_sgf")).unwrap();

    // Test natural sort order by iterating through the playlist
    let first = playlist
        .current()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert_eq!(first, "game1");

    assert!(playlist.next());
    let second = playlist
        .current()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert_eq!(second, "game2");

    assert!(playlist.next());
    let third = playlist
        .current()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert_eq!(third, "game10");

    assert!(playlist.next());
    let fourth = playlist
        .current()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert_eq!(fourth, "game20");

    // Should be at the end now
    assert_eq!(playlist.has_next(), false);
}

#[test]
fn test_folder_playlist() {
    let playlist = PlaylistManager::new(Some("test_sgf")).unwrap();

    assert_eq!(playlist.is_single_file(), false);
    assert_eq!(playlist.has_next(), true);

    let first = playlist.current().to_string_lossy().to_string();
    assert!(first.contains("game1"));

    let mut playlist = playlist;
    playlist.next();
    let second = playlist.current().to_string_lossy().to_string();
    assert!(second.contains("game2"));
}

#[test]
fn test_playlist_reset() {
    let mut playlist = PlaylistManager::new(Some("test_sgf")).unwrap();

    let initial = playlist
        .current()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    playlist.next();
    playlist.next();

    playlist.reset();
    let after_reset = playlist
        .current()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    assert_eq!(initial, after_reset);
    assert_eq!(after_reset, "game1");
}
