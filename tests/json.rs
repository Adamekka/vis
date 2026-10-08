use std::{fs, process::Command};

#[test]
fn invalid_json_is_not_overwritten() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("library.json");
    fs::write(&path, "{invalid json").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_vis"))
        .args(["--storage", "json", "--file"])
        .arg(&path)
        .arg("players")
        .env_remove("DATABASE_URL")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("Invalid JSON library"));
    assert_eq!(error.contains("Details:"), cfg!(debug_assertions));
    assert_eq!(fs::read_to_string(path).unwrap(), "{invalid json");
}

#[test]
fn json_requires_a_file_path() {
    let output = Command::new(env!("CARGO_BIN_EXE_vis"))
        .args(["--storage", "json", "players"])
        .env_remove("DATABASE_URL")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--file"));
}

#[test]
fn invalid_domain_values_in_existing_json_are_not_overwritten() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("library.json");
    let valid = serde_json::json!({
        "players": [{"id": 1, "username": "alex", "email": "alex@example.test"}],
        "games": [{"id": 1, "title": "Game", "release_date": "2024-02-29"}],
        "genres": [{"id": 1, "name": "Adventure"}],
        "game_genres": [{"game_id": 1, "genre_id": 1}],
        "player_games": [{"player_id": 1, "game_id": 1, "status": "not_started", "playtime_minutes": 0, "added_at": "2024-03-01T00:00:00Z"}],
        "achievements": [{"id": 1, "game_id": 1, "name": "First step", "description": "Finish the tutorial", "points": 0}],
        "player_achievements": []
    });
    for (field, value) in [
        ("/players/0/username", serde_json::json!(" \t")),
        ("/players/0/email", serde_json::json!("")),
        ("/games/0/title", serde_json::json!("")),
        ("/games/0/release_date", serde_json::json!("2025-02-29")),
        ("/genres/0/name", serde_json::json!("")),
        ("/player_games/0/playtime_minutes", serde_json::json!(-1)),
        ("/achievements/0/name", serde_json::json!("")),
        ("/achievements/0/description", serde_json::json!("")),
        ("/achievements/0/points", serde_json::json!(-1)),
    ] {
        let mut invalid = valid.clone();
        *invalid.pointer_mut(field).unwrap() = value;
        let json = serde_json::to_string(&invalid).unwrap();
        fs::write(&path, &json).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_vis"))
            .args(["--storage", "json", "--file"])
            .arg(&path)
            .arg("players")
            .env_remove("DATABASE_URL")
            .output()
            .unwrap();
        assert!(!output.status.success(), "Accepted invalid field {field}");
        assert_eq!(fs::read_to_string(&path).unwrap(), json);
    }
}
