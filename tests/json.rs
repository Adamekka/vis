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
