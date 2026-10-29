use super::models::{PlayerGame, Response};

impl Response {
    pub fn text(self) -> String {
        fn library_entries(entries: &[PlayerGame]) -> String {
            let mut text = "Player ID\tGame ID\tTitle\tStatus\tMinutes\tAdded at\n".to_owned();
            for entry in entries {
                text.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\n",
                    entry.player_id,
                    entry.game_id,
                    entry.title,
                    entry.status,
                    entry.playtime_minutes,
                    entry.added_at
                ));
            }
            text
        }

        match self {
            Self::AllItems(items) => format!(
                "Players\n{}\nGames\n{}\nGenres\n{}\nGenre assignments\nGame ID\tGenre ID\n{}\nLibrary entries\n{}\nAchievements\n{}\nUnlocked achievements\n{}",
                Self::Players(items.players).text(),
                Self::Games(items.games).text(),
                Self::Genres(items.genres).text(),
                items
                    .game_genres
                    .into_iter()
                    .map(|tag| format!("{}\t{}\n", tag.game_id, tag.genre_id))
                    .collect::<String>(),
                library_entries(&items.library),
                Self::Achievements(items.achievements).text(),
                Self::Unlocked(items.unlocked).text(),
            ),
            Self::Created(id) => format!("{id}\n"),
            Self::GameAddedToLibrary {
                player_id,
                game_id,
                genre_ids,
                achievement_ids,
            } => format!(
                "Game ID: {game_id}\nAssigned genre IDs: {genre_ids:?}\nCreated achievement IDs: {achievement_ids:?}\nGame added to player {player_id}'s library.\n"
            ),
            Self::Players(players) => {
                let mut text = "ID\tUsername\tEmail\n".to_owned();
                for player in players {
                    text.push_str(&format!(
                        "{}\t{}\t{}\n",
                        player.id, player.username, player.email
                    ));
                }
                text
            }
            Self::Games(games) => {
                let mut text = "ID\tTitle\tReleased\tGenres\n".to_owned();
                for game in games {
                    text.push_str(&format!(
                        "{}\t{}\t{}\t{}\n",
                        game.id, game.title, game.release_date, game.genres
                    ));
                }
                text
            }
            Self::Genres(genres) => {
                let mut text = "ID\tName\n".to_owned();
                for genre in genres {
                    text.push_str(&format!("{}\t{}\n", genre.id, genre.name));
                }
                text
            }
            Self::GenreAssigned => "Genre assigned.\n".to_owned(),
            Self::AddedToLibrary => "Game added to library.\n".to_owned(),
            Self::Library(library) => {
                let mut text = format!(
                    "Player\n{}\nGames\n",
                    Self::Players(vec![library.player]).text(),
                );
                if library.entries.is_empty() {
                    text.push_str(&library_entries(&[]));
                    text.push_str("No games in this player's library.\n");
                }
                for entry in &library.entries {
                    // Repeat game labels because the achievement table separates consecutive games.
                    text.push_str(&library_entries(std::slice::from_ref(entry)));
                    if entry.achievements.is_empty() {
                        text.push_str("  No achievements for this game.\n");
                    } else {
                        text.push_str("  Achievements\n  ID\tName\tDescription\tPoints\tStatus\n");
                        for item in &entry.achievements {
                            let achievement = &item.achievement;
                            text.push_str(&format!(
                                "  {}\t{}\t{}\t{}\t{}\n",
                                achievement.id,
                                achievement.name,
                                achievement.description,
                                achievement.points,
                                item.status,
                            ));
                        }
                    }
                }
                text
            }
            Self::ProgressUpdated => "Progress updated.\n".to_owned(),
            Self::Achievements(achievements) => {
                let mut text = "ID\tGame ID\tName\tDescription\tPoints\n".to_owned();
                for achievement in achievements {
                    text.push_str(&format!(
                        "{}\t{}\t{}\t{}\t{}\n",
                        achievement.id,
                        achievement.game_id,
                        achievement.name,
                        achievement.description,
                        achievement.points
                    ));
                }
                text
            }
            Self::AchievementUnlocked => "Achievement unlocked.\n".to_owned(),
            Self::Unlocked(achievements) => {
                let mut text =
                    "Player ID\tID\tGame ID\tGame\tAchievement\tPoints\tUnlocked at\n".to_owned();
                for achievement in achievements {
                    text.push_str(&format!(
                        "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                        achievement.player_id,
                        achievement.achievement_id,
                        achievement.game_id,
                        achievement.game_title,
                        achievement.name,
                        achievement.points,
                        achievement.unlocked_at
                    ));
                }
                text
            }
        }
    }
}
