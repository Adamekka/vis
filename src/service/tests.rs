use std::fs;

use crate::{
    app_error::AppError,
    data::{JsonStorage, PostgresStorage},
    domain::{CurrentUser, Storage},
};

use super::{
    LibraryService, Service,
    models::{Email, NewAchievement, Request, Response, Status},
};

fn signed_in_service<S: Storage>(storage: S, username: &str) -> LibraryService<S> {
    let mut service = LibraryService::new(storage, CurrentUser::default());
    service.sign_in(username, username).unwrap();
    service
}

#[test]
fn invalid_service_requests_are_rejected_before_storage_changes() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("library.json");
    let mut service = signed_in_service(JsonStorage::open(&path).unwrap(), "admin");
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
        Request::AddGameToLibrary {
            player_id: 1,
            title: "Game".to_owned(),
            release_date: "2024-02-29".to_owned(),
            genre_ids: vec![1],
            achievements: vec![NewAchievement {
                name: "First step".to_owned(),
                description: "Finish the tutorial".to_owned(),
                points: -1,
            }],
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
    assert!(matches!(
        service.execute(Request::AddGameToLibrary {
            player_id: 1,
            title: "Game".to_owned(),
            release_date: "2024-02-29".to_owned(),
            genre_ids: vec![],
            achievements: vec![],
        }),
        Err(AppError::EmptyGameSetup)
    ));
    assert_eq!(fs::read(&path).unwrap(), original);
}

#[test]
fn roles_limit_actions_and_player_emails() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("library.json");
    let current_user = CurrentUser::default();
    let mut service = LibraryService::new(JsonStorage::open(&path).unwrap(), current_user.clone());
    assert!(matches!(
        service.execute(Request::Players),
        Err(AppError::SignInRequired)
    ));
    assert!(matches!(
        service.sign_in("admin", "wrong"),
        Err(AppError::InvalidCredentials)
    ));

    service.sign_in("admin", "admin").unwrap();
    for (username, email) in [("alex", "alex@example.test"), ("sam", "sam@example.test")] {
        service
            .execute(Request::AddPlayer {
                username: username.to_owned(),
                email: email.to_owned(),
            })
            .unwrap();
    }
    let Ok(Response::Players(players)) = service.execute(Request::Players) else {
        panic!("expected players");
    };
    assert!(
        players
            .iter()
            .all(|player| matches!(player.email, Email::Visible(_)))
    );

    // The injected handle is shared, so signing in through any clone changes the service's user.
    service.sign_out();
    current_user.sign_in(crate::domain::authenticate("alex", "alex").unwrap());
    let Ok(Response::Players(players)) = service.execute(Request::Players) else {
        panic!("expected players");
    };
    let emails: Vec<_> = players
        .iter()
        .map(|player| match &player.email {
            Email::Visible(email) => email.as_str(),
            Email::Hidden => "hidden",
        })
        .collect();
    assert_eq!(emails, ["alex@example.test", "hidden"]);
    assert!(service.execute(Request::Library { player_id: 1 }).is_ok());
    let original = fs::read(&path).unwrap();
    for request in [
        Request::Library { player_id: 2 },
        Request::Unlocked { player_id: 2 },
        Request::AddToLibrary {
            player_id: 2,
            game_id: 1,
        },
        Request::AddGenre {
            name: "Adventure".to_owned(),
        },
    ] {
        assert!(matches!(service.execute(request), Err(AppError::Forbidden)));
    }
    assert_eq!(fs::read(&path).unwrap(), original);
}

fn exercise_transaction_recovery(storage: impl Storage) {
    let mut service = signed_in_service(storage, "admin");
    let Response::Created(player_id) = service
        .execute(Request::AddPlayer {
            username: "alex".to_owned(),
            email: "alex@example.test".to_owned(),
        })
        .unwrap()
    else {
        panic!("expected a player ID");
    };
    let Response::Created(genre_id) = service
        .execute(Request::AddGenre {
            name: "Adventure".to_owned(),
        })
        .unwrap()
    else {
        panic!("expected a genre ID");
    };

    for (target_player, genres, second_name, succeeds) in [
        (player_id, vec![genre_id, 999999], "Explorer", false),
        (player_id, vec![genre_id], "First step", false),
        (999999, vec![genre_id], "Explorer", false),
        (player_id, vec![genre_id], "Explorer", true),
    ] {
        let result = service.execute(Request::AddGameToLibrary {
            player_id: target_player,
            title: "Game".to_owned(),
            release_date: "2024-02-29".to_owned(),
            genre_ids: genres,
            achievements: ["First step", second_name]
                .into_iter()
                .map(|name| NewAchievement {
                    name: name.to_owned(),
                    description: "Finish a task".to_owned(),
                    points: 10,
                })
                .collect(),
        });
        if succeeds {
            let Response::GameAddedToLibrary {
                game_id,
                achievement_ids,
                ..
            } = result.unwrap()
            else {
                panic!("expected the created game and achievement IDs");
            };
            assert_eq!(achievement_ids.len(), 2);
            assert_eq!(service.storage.achievements(game_id).unwrap().len(), 2);
        } else {
            assert!(result.is_err());
        }
        assert_eq!(
            service.storage.games().unwrap().len(),
            usize::from(succeeds)
        );
        assert_eq!(
            service.storage.game_genres().unwrap().len(),
            usize::from(succeeds)
        );
        assert_eq!(
            service.storage.library(player_id).unwrap().entries.len(),
            usize::from(succeeds)
        );
    }
}

#[test]
fn json_service_can_commit_after_rollback() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("library.json");
    exercise_transaction_recovery(JsonStorage::open(&path).unwrap());
    let mut reopened = JsonStorage::open(&path).unwrap();
    assert_eq!(reopened.games().unwrap().len(), 1);
}

#[test]
fn json_commit_failure_preserves_memory_and_destination() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("library.json");
    let mut storage = JsonStorage::open(&path).unwrap();
    let original = fs::read(&path).unwrap();
    let backup = directory.path().join("original.json");
    fs::rename(&path, &backup).unwrap();
    // A directory at the destination forces commit to fail without depending on user permissions.
    fs::create_dir(&path).unwrap();
    let sentinel = path.join("keep.txt");
    fs::write(&sentinel, "keep").unwrap();

    let result = storage.transaction(|storage| {
        storage.add_game(
            crate::domain::NewGame::try_from(("Game".to_owned(), "2024-02-29".to_owned())).unwrap(),
        )
    });
    assert!(result.is_err());
    assert!(storage.games().unwrap().is_empty());
    assert_eq!(fs::read_to_string(&sentinel).unwrap(), "keep");
    assert_eq!(fs::read(&backup).unwrap(), original);

    fs::remove_file(sentinel).unwrap();
    fs::remove_dir(&path).unwrap();
    fs::rename(backup, &path).unwrap();
    exercise_transaction_recovery(storage);
    let mut reopened = JsonStorage::open(&path).unwrap();
    assert_eq!(reopened.games().unwrap().len(), 1);
}

#[test]
#[ignore = "requires TEST_DATABASE_URL pointing to a running PostgreSQL database"]
fn postgres_service_can_commit_after_rollback() {
    use std::{env, panic, time::SystemTime};

    use postgres::{Client, NoTls};

    let database_url = env::var("TEST_DATABASE_URL").expect("Set TEST_DATABASE_URL explicitly");
    let config: postgres::Config = database_url.parse().unwrap();
    assert!(
        config.get_options().is_none(),
        "Test sets its own search_path"
    );
    let mut admin = config.connect(NoTls).unwrap();
    let nonce = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let schema = format!("vis_transaction_test_{}_{nonce}", std::process::id());
    admin
        .batch_execute(&format!("CREATE SCHEMA {schema}"))
        .unwrap();
    let scoped_url =
        if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
            let separator = if database_url.contains('?') { '&' } else { '?' };
            format!("{database_url}{separator}options=-csearch_path%3D{schema}")
        } else {
            format!("{database_url} options='-csearch_path={schema}'")
        };
    // A private schema keeps this test isolated, and catch_unwind ensures cleanup after assertion failures.
    let result = panic::catch_unwind(|| {
        let mut db = Client::connect(&scoped_url, NoTls).unwrap();
        db.batch_execute(include_str!("../../schema.sql")).unwrap();
        exercise_transaction_recovery(PostgresStorage::connect(&scoped_url).unwrap());
    });
    admin
        .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
        .unwrap();
    if let Err(payload) = result {
        panic::resume_unwind(payload);
    }
}
