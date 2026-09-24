use std::{env, panic, process::Command, time::SystemTime};

use postgres::{Client, NoTls, error::SqlState};

#[test]
fn help_and_configuration_errors() {
    let help = Command::new(env!("CARGO_BIN_EXE_vis"))
        .arg("--help")
        .env_remove("DATABASE_URL")
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("add-to-library"));

    for value in [None, Some(""), Some("   ")] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_vis"));
        command.arg("players").env_remove("DATABASE_URL");
        if let Some(value) = value {
            command.env("DATABASE_URL", value);
        }
        let output = command.output().unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Set DATABASE_URL"));
    }

    let failure = Command::new(env!("CARGO_BIN_EXE_vis"))
        .arg("players")
        .env("DATABASE_URL", "invalid-connection-string")
        .output()
        .unwrap();
    assert!(!failure.status.success());
    let error = String::from_utf8(failure.stderr).unwrap();
    assert!(error.contains("Could not connect to PostgreSQL"));
    assert_eq!(error.contains("Details:"), cfg!(debug_assertions));
}

#[test]
#[ignore = "requires TEST_DATABASE_URL pointing to a running PostgreSQL database"]
fn postgres_workflow() {
    let database_url = env::var("TEST_DATABASE_URL").expect("Set TEST_DATABASE_URL explicitly");
    assert!(!database_url.trim().is_empty());
    let config: postgres::Config = database_url.parse().unwrap();
    assert!(
        config.get_options().is_none(),
        "Test sets its own search_path; omit connection options"
    );
    let mut admin = config.connect(NoTls).unwrap();
    let nonce = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let schema = format!("vis_test_{}_{nonce}", std::process::id());
    admin
        .batch_execute(&format!("CREATE SCHEMA {schema}"))
        .unwrap();
    // Each CLI process shares only this test's schema, leaving existing application data untouched.
    let scoped_url =
        if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
            let separator = if database_url.contains('?') { '&' } else { '?' };
            format!("{database_url}{separator}options=-csearch_path%3D{schema}")
        } else {
            format!("{database_url} options='-csearch_path={schema}'")
        };

    // Clean up the private schema even if a workflow assertion fails.
    let result = panic::catch_unwind(|| {
        let mut db = Client::connect(&scoped_url, NoTls).unwrap();
        db.batch_execute(include_str!("../schema.sql")).unwrap();
        let table_count: i64 = db.query_one(
            "SELECT count(*) FROM information_schema.tables WHERE table_schema = current_schema()",
            &[],
        ).unwrap().get(0);
        assert_eq!(table_count, 7);

        let run = |args: &[&str], success: bool| {
            let output = Command::new(env!("CARGO_BIN_EXE_vis"))
                .args(args)
                .env("DATABASE_URL", &scoped_url)
                .output()
                .unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert_eq!(
                output.status.success(),
                success,
                "{args:?}\n{stdout}\n{stderr}"
            );
            if success {
                stdout.trim_end().to_owned()
            } else {
                stderr
            }
        };

        assert_eq!(run(&["players"], true), "ID\tUsername\tEmail");
        let player = run(&["add-player", "alex", "alex@example.test"], true);
        let other_player = run(&["add-player", "sam", "sam@example.test"], true);
        assert!(run(&["players"], true).contains("alex\talex@example.test"));
        assert_eq!(
            run(&["library", &player], true),
            "Player ID\tGame ID\tTitle\tStatus\tMinutes\tAdded at"
        );
        assert_eq!(
            run(&["unlocked", &player], true),
            "Player ID\tID\tGame ID\tGame\tAchievement\tPoints\tUnlocked at"
        );
        assert!(
            run(&["add-player", "alex", "other@example.test"], false).contains("already exists")
        );
        assert!(
            run(&["add-player", "other", "alex@example.test"], false).contains("already exists")
        );
        assert!(run(&["add-player", "   ", "blank@example.test"], false).contains("Invalid value"));

        // Quotes and SQL-like text must remain data in parameterized statements.
        let title = "Player's game'; DROP TABLE game; --";
        let game = run(&["add-game", title, "2020-01-02"], true);
        let other_game = run(&["add-game", "Another game", "2021-02-03"], true);
        assert!(run(&["games"], true).contains(&format!("{title}\t2020-01-02\t")));
        assert!(
            run(&["add-game", "Invalid date", "2025-02-30"], false)
                .contains("Invalid release date")
        );
        assert!(
            run(&["add-game", "Ambiguous date", "tomorrow"], false)
                .contains("Invalid release date")
        );
        assert!(run(&["add-genre", "\t\n"], false).contains("Invalid value"));
        let genre = run(&["add-genre", "Adventure"], true);
        assert!(run(&["add-genre", "Adventure"], false).contains("already exists"));
        let other_genre = run(&["add-genre", "Puzzle"], true);
        run(&["tag-game", &game, &genre], true);
        run(&["tag-game", &game, &other_genre], true);
        run(&["tag-game", &other_game, &genre], true);
        assert!(run(&["games"], true).contains(&format!("{title}\t2020-01-02\tAdventure, Puzzle")));
        assert!(run(&["genres"], true).contains("Adventure"));
        assert!(run(&["tag-game", &game, &genre], false).contains("already exists"));
        run(&["tag-game", &game, "999999"], false);

        let achievement = run(
            &[
                "add-achievement",
                &game,
                "First step",
                "Finish the tutorial",
                "10",
            ],
            true,
        );
        let other_achievement = run(
            &[
                "add-achievement",
                &other_game,
                "First step",
                "Finish the game",
                "20",
            ],
            true,
        );
        assert!(
            run(
                &["add-achievement", &game, "First step", "Same name", "10"],
                false
            )
            .contains("already exists")
        );
        assert!(
            run(&["achievements", &game], true).contains("First step\tFinish the tutorial\t10")
        );
        assert!(
            run(&["unlock", &player, &achievement], false).contains("not in the player's library")
        );
        run(&["add-to-library", &player, &game], true);
        run(&["add-to-library", &other_player, &game], true);
        assert!(run(&["library", &player], true).contains("not_started\t0"));
        assert!(run(&["add-to-library", &player, &game], false).contains("already exists"));
        run(&["set-progress", &player, &game, "playing", "120"], true);
        assert!(run(&["library", &player], true).contains("playing\t120"));
        assert!(run(&["library", &other_player], true).contains("not_started\t0"));
        run(&["set-progress", &player, &game, "unknown", "5"], false);
        run(
            &["set-progress", &player, &game, "playing", "--", "-1"],
            false,
        );
        run(
            &[
                "add-achievement",
                &game,
                "Invalid",
                "Negative points",
                "--",
                "-1",
            ],
            false,
        );
        run(
            &["set-progress", &player, &other_game, "playing", "5"],
            false,
        );
        assert!(
            run(&["set-progress", "999999", &game, "playing", "5"], false)
                .contains("Check the player and game IDs")
        );

        run(&["unlock", &player, &achievement], true);
        assert!(run(&["unlocked", &player], true).contains("First step\t10\t"));
        assert!(!run(&["unlocked", &other_player], true).contains("First step"));
        assert!(run(&["unlock", &player, &achievement], false).contains("already exists"));
        run(&["unlock", &player, &other_achievement], false);
        run(&["unlock", &player, "999999"], false);
        run(&["library", "999999"], false);
        run(&["unlocked", "999999"], false);
        run(&["achievements", "999999"], false);
        run(&["add-to-library", &player, "999999"], false);
        run(&["add-to-library", "999999", &game], false);

        // Bypassing the CLI must not allow mismatched games, invalid progress, or negative points.
        let player_id: i64 = player.parse().unwrap();
        let game_id: i64 = game.parse().unwrap();
        let other_achievement_id: i64 = other_achievement.parse().unwrap();
        let error = db.execute(
            "INSERT INTO player_achievement (player_id, achievement_id, game_id) VALUES ($1, $2, $3)",
            &[&player_id, &other_achievement_id, &game_id],
        ).unwrap_err();
        assert_eq!(error.code(), Some(&SqlState::FOREIGN_KEY_VIOLATION));
        for sql in [
            "UPDATE player_game SET status = 'invalid'",
            "UPDATE player_game SET playtime_minutes = -1",
            "UPDATE achievement SET points = -1",
        ] {
            assert_eq!(
                db.execute(sql, &[]).unwrap_err().code(),
                Some(&SqlState::CHECK_VIOLATION)
            );
        }
    });
    admin
        .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
        .unwrap();
    if let Err(payload) = result {
        panic::resume_unwind(payload);
    }
}
