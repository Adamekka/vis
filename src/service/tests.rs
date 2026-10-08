use std::fs;

use crate::{app_error::AppError, data::JsonStorage};

use super::{
    LibraryService, Service,
    models::{Request, Status},
};

#[test]
fn invalid_service_requests_are_rejected_before_storage_changes() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("library.json");
    let mut service = LibraryService::new(JsonStorage::open(&path).unwrap());
    let original = fs::read(&path).unwrap();
    for request in [
        Request::AddPlayer {
            username: " ".to_owned(),
            email: "alex@example.test".to_owned(),
        },
        Request::AddPlayer {
            username: "alex".to_owned(),
            email: "".to_owned(),
        },
        Request::AddGame {
            title: "".to_owned(),
            release_date: "2024-02-29".to_owned(),
        },
        Request::AddGenre {
            name: "\t".to_owned(),
        },
        Request::SetProgress {
            player_id: 1,
            game_id: 2,
            status: Status::Playing,
            playtime_minutes: -1,
        },
        Request::AddAchievement {
            game_id: 2,
            name: "First step".to_owned(),
            description: "Finish the tutorial".to_owned(),
            points: -1,
        },
    ] {
        assert!(matches!(
            service.execute(request),
            Err(AppError::InvalidValue)
        ));
        assert_eq!(fs::read(&path).unwrap(), original);
    }
    assert!(matches!(
        service.execute(Request::AddGame {
            title: "Game".to_owned(),
            release_date: "2025-02-29".to_owned()
        }),
        Err(AppError::InvalidDate)
    ));
    assert_eq!(fs::read(&path).unwrap(), original);
}
