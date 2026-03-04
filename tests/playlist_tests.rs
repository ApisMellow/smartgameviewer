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

#[test]
fn test_playlist_nonexistent_path() {
    let result = PlaylistManager::new(Some("/tmp/does_not_exist_xyz_smartgameviewer"));
    assert!(result.is_err());
}

#[test]
fn test_playlist_empty_directory() {
    // Create a temp directory with no .sgf files
    let dir = std::env::temp_dir().join("smartgameviewer_empty_test");
    std::fs::create_dir_all(&dir).unwrap();
    // Make sure no .sgf files exist
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|e| e == "sgf").unwrap_or(false) {
            std::fs::remove_file(path).unwrap();
        }
    }

    let result = PlaylistManager::new(Some(dir.to_str().unwrap()));
    assert!(result.is_err());

    // Clean up
    let _ = std::fs::remove_dir(&dir);
}

#[test]
fn test_peek_next() {
    let mut playlist = PlaylistManager::new(Some("test_sgf")).unwrap();

    // peek_next should return second file without advancing
    let peeked = playlist.peek_next();
    assert!(peeked.is_some());
    let peeked_name = peeked
        .unwrap()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert_eq!(peeked_name, "game2");

    // Current should still be game1
    let current_name = playlist
        .current()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert_eq!(current_name, "game1");

    // Advance to end
    while playlist.next() {}

    // peek_next at end should return None
    assert_eq!(playlist.peek_next(), None);
}

#[test]
fn test_next_returns_false_at_end() {
    let mut playlist = PlaylistManager::new(Some("test_sgf")).unwrap();

    // Exhaust playlist
    while playlist.next() {}

    // Further next() should return false
    assert_eq!(playlist.next(), false);
    assert_eq!(playlist.next(), false);
}
