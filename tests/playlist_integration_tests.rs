use smartgameviewer::playlist::PlaylistManager;

#[test]
fn test_playlist_with_real_sgf_files() {
    // Test with actual sgf folder if it exists
    if let Ok(playlist) = PlaylistManager::new(Some("sgf")) {
        assert!(!playlist.is_empty());

        // Should be able to get current file
        let current = playlist.current();
        assert!(current.extension().unwrap() == "sgf");
    }
}

#[test]
fn test_single_file_mode_compatibility() {
    // Test that providing a single file still works
    if let Ok(playlist) = PlaylistManager::new(Some("sgf/AlphaGo_LeeSedol_game4.sgf")) {
        assert!(playlist.is_single_file());
        assert!(!playlist.has_next());
    }
}
