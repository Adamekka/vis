use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    app_error::AppError,
    domain::{
        Achievement, Game, GameGenre, Genre, NewAchievement, NewGame, NewGenre, NewPlayer, Player,
        PlayerAchievement, PlayerGame, Progress, Status, Storage,
    },
};

pub struct JsonStorage {
    path: PathBuf,
    data: Snapshot,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    players: Vec<StoredPlayer>,
    games: Vec<StoredGame>,
    genres: Vec<StoredGenre>,
    game_genres: Vec<StoredGameGenre>,
    player_games: Vec<StoredPlayerGame>,
    achievements: Vec<StoredAchievement>,
    player_achievements: Vec<StoredPlayerAchievement>,
}

// Persist the normalized records. Titles, genre names, and achievement details in list
// results are joined at read time so the file never contains conflicting copies of them.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredPlayer {
    id: i64,
    username: String,
    email: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredGame {
    id: i64,
    title: String,
    release_date: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredGenre {
    id: i64,
    name: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredGameGenre {
    game_id: i64,
    genre_id: i64,
}

// Serde's remote derive keeps the JSON representation out of the domain enum.
#[derive(Deserialize, Serialize)]
#[serde(remote = "Status", rename_all = "snake_case")]
enum StoredStatus {
    NotStarted,
    Playing,
    Completed,
    Abandoned,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredPlayerGame {
    player_id: i64,
    game_id: i64,
    #[serde(with = "StoredStatus")]
    status: Status,
    playtime_minutes: i32,
    added_at: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredAchievement {
    id: i64,
    game_id: i64,
    name: String,
    description: String,
    points: i32,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredPlayerAchievement {
    player_id: i64,
    achievement_id: i64,
    game_id: i64,
    unlocked_at: String,
}

impl Snapshot {
    fn validate(&self) -> Result<(), AppError> {
        fn ids(values: impl Iterator<Item = i64>) -> Result<HashSet<i64>, AppError> {
            let mut ids = HashSet::new();
            for id in values {
                if id <= 0 || !ids.insert(id) {
                    return Err(AppError::InvalidFile("IDs must be positive and unique."));
                }
            }
            Ok(ids)
        }

        let player_ids = ids(self.players.iter().map(|player| player.id))?;
        let game_ids = ids(self.games.iter().map(|game| game.id))?;
        let genre_ids = ids(self.genres.iter().map(|genre| genre.id))?;
        ids(self.achievements.iter().map(|achievement| achievement.id))?;

        let mut usernames = HashSet::new();
        let mut emails = HashSet::new();
        for player in &self.players {
            NewPlayer::new(player.username.clone(), player.email.clone())?;
            if !usernames.insert(&player.username) || !emails.insert(&player.email) {
                return Err(AppError::AlreadyExists);
            }
        }
        for game in &self.games {
            NewGame::new(game.title.clone(), game.release_date.clone())?;
        }
        let mut genre_names = HashSet::new();
        for genre in &self.genres {
            NewGenre::new(genre.name.clone())?;
            if !genre_names.insert(&genre.name) {
                return Err(AppError::AlreadyExists);
            }
        }
        let mut tags = HashSet::new();
        for tag in &self.game_genres {
            if !game_ids.contains(&tag.game_id) || !genre_ids.contains(&tag.genre_id) {
                return Err(AppError::InvalidReference);
            }
            if !tags.insert((tag.game_id, tag.genre_id)) {
                return Err(AppError::AlreadyExists);
            }
        }
        let mut library = HashSet::new();
        for entry in &self.player_games {
            if !player_ids.contains(&entry.player_id) || !game_ids.contains(&entry.game_id) {
                return Err(AppError::InvalidReference);
            }
            Progress::new(entry.status, entry.playtime_minutes)?;
            if !library.insert((entry.player_id, entry.game_id)) {
                return Err(AppError::AlreadyExists);
            }
        }
        let mut achievement_names = HashSet::new();
        for achievement in &self.achievements {
            if !game_ids.contains(&achievement.game_id) {
                return Err(AppError::InvalidReference);
            }
            NewAchievement::new(
                achievement.name.clone(),
                achievement.description.clone(),
                achievement.points,
            )?;
            if !achievement_names.insert((achievement.game_id, &achievement.name)) {
                return Err(AppError::AlreadyExists);
            }
        }
        let mut unlocks = HashSet::new();
        for unlock in &self.player_achievements {
            if !library.contains(&(unlock.player_id, unlock.game_id))
                || !self.achievements.iter().any(|achievement| {
                    achievement.id == unlock.achievement_id && achievement.game_id == unlock.game_id
                })
            {
                return Err(AppError::InvalidReference);
            }
            if !unlocks.insert((unlock.player_id, unlock.achievement_id)) {
                return Err(AppError::AlreadyExists);
            }
        }
        Ok(())
    }
}

impl JsonStorage {
    pub fn open(path: &Path) -> Result<Self, AppError> {
        let (data, created) = match fs::read_to_string(path) {
            Ok(json) => (serde_json::from_str(&json)?, false),
            Err(error) if error.kind() == io::ErrorKind::NotFound => (
                Snapshot {
                    players: Vec::new(),
                    games: Vec::new(),
                    genres: Vec::new(),
                    game_genres: Vec::new(),
                    player_games: Vec::new(),
                    achievements: Vec::new(),
                    player_achievements: Vec::new(),
                },
                true,
            ),
            Err(error) => return Err(error.into()),
        };
        let storage = Self {
            path: path.to_owned(),
            data,
        };
        storage.data.validate()?;
        if created {
            storage.write(&storage.data)?;
        }
        Ok(storage)
    }

    fn write(&self, data: &Snapshot) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    fn update<R>(
        &mut self,
        change: impl FnOnce(&mut Snapshot) -> Result<R, AppError>,
    ) -> Result<R, AppError> {
        // Validate a candidate before saving so rejected input does not change the library.
        let mut candidate = self.data.clone();
        let result = change(&mut candidate)?;
        candidate.validate()?;
        self.write(&candidate)?;
        self.data = candidate;
        Ok(result)
    }

    fn require_player(&self, id: i64) -> Result<(), AppError> {
        if !self.data.players.iter().any(|player| player.id == id) {
            return Err(AppError::NotFound(
                "Player not found. Use players to list IDs.",
            ));
        }
        Ok(())
    }
}

// There is no deletion operation, so the largest stored ID is sufficient to keep IDs increasing.
fn next_id(ids: impl Iterator<Item = i64>) -> Result<i64, AppError> {
    ids.max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(AppError::IdExhausted)
}

impl Storage for JsonStorage {
    fn add_player(&mut self, player: NewPlayer) -> Result<i64, AppError> {
        let (username, email) = player.into_parts();
        self.update(|data| {
            let id = next_id(data.players.iter().map(|player| player.id))?;
            data.players.push(StoredPlayer {
                id,
                username,
                email,
            });
            Ok(id)
        })
    }

    fn players(&mut self) -> Result<Vec<Player>, AppError> {
        let mut players: Vec<_> = self
            .data
            .players
            .iter()
            .map(|player| Player {
                id: player.id,
                username: player.username.clone(),
                email: player.email.clone(),
            })
            .collect();
        players.sort_by_key(|player| player.id);
        Ok(players)
    }

    fn add_game(&mut self, game: NewGame) -> Result<i64, AppError> {
        let (title, release_date) = game.into_parts();
        self.update(|data| {
            let id = next_id(data.games.iter().map(|game| game.id))?;
            data.games.push(StoredGame {
                id,
                title,
                release_date,
            });
            Ok(id)
        })
    }

    fn games(&mut self) -> Result<Vec<Game>, AppError> {
        let mut games: Vec<_> = self
            .data
            .games
            .iter()
            .map(|game| {
                let mut genres: Vec<_> = self
                    .data
                    .game_genres
                    .iter()
                    .filter(|tag| tag.game_id == game.id)
                    .map(|tag| {
                        self.data
                            .genres
                            .iter()
                            .find(|genre| genre.id == tag.genre_id)
                            .expect("validated genre reference")
                            .name
                            .clone()
                    })
                    .collect();
                genres.sort();
                Game {
                    id: game.id,
                    title: game.title.clone(),
                    release_date: game.release_date.clone(),
                    genres,
                }
            })
            .collect();
        games.sort_by_key(|game| game.id);
        Ok(games)
    }

    fn add_genre(&mut self, genre: NewGenre) -> Result<i64, AppError> {
        let name = genre.into_name();
        self.update(|data| {
            let id = next_id(data.genres.iter().map(|genre| genre.id))?;
            data.genres.push(StoredGenre { id, name });
            Ok(id)
        })
    }

    fn genres(&mut self) -> Result<Vec<Genre>, AppError> {
        let mut genres: Vec<_> = self
            .data
            .genres
            .iter()
            .map(|genre| Genre {
                id: genre.id,
                name: genre.name.clone(),
            })
            .collect();
        genres.sort_by_key(|genre| genre.id);
        Ok(genres)
    }

    fn tag_game(&mut self, tag: GameGenre) -> Result<(), AppError> {
        self.update(|data| {
            data.game_genres.push(StoredGameGenre {
                game_id: tag.game_id,
                genre_id: tag.genre_id,
            });
            Ok(())
        })
    }

    fn add_to_library(
        &mut self,
        player_id: i64,
        game_id: i64,
        progress: Progress,
    ) -> Result<(), AppError> {
        let (status, playtime_minutes) = progress.into_parts();
        self.update(|data| {
            data.player_games.push(StoredPlayerGame {
                player_id,
                game_id,
                status,
                playtime_minutes,
                added_at: Utc::now().to_rfc3339(),
            });
            Ok(())
        })
    }

    fn owns_game(&mut self, player_id: i64, game_id: i64) -> Result<bool, AppError> {
        Ok(self
            .data
            .player_games
            .iter()
            .any(|entry| entry.player_id == player_id && entry.game_id == game_id))
    }

    fn library(&mut self, player_id: i64) -> Result<Vec<PlayerGame>, AppError> {
        self.require_player(player_id)?;
        let mut library: Vec<_> = self
            .data
            .player_games
            .iter()
            .filter(|entry| entry.player_id == player_id)
            .map(|entry| PlayerGame {
                player_id: entry.player_id,
                game_id: entry.game_id,
                title: self
                    .data
                    .games
                    .iter()
                    .find(|game| game.id == entry.game_id)
                    .expect("validated game reference")
                    .title
                    .clone(),
                status: entry.status.to_string(),
                playtime_minutes: entry.playtime_minutes,
                added_at: entry.added_at.clone(),
            })
            .collect();
        library.sort_by_key(|entry| entry.game_id);
        Ok(library)
    }

    fn set_progress(
        &mut self,
        player_id: i64,
        game_id: i64,
        progress: Progress,
    ) -> Result<(), AppError> {
        let (status, playtime_minutes) = progress.into_parts();
        self.update(|data| {
            let entry = data.player_games.iter_mut()
                .find(|entry| entry.player_id == player_id && entry.game_id == game_id)
                .ok_or(AppError::NotFound("Game not found in this player's library. Check the player and game IDs, or use add-to-library."))?;
            entry.status = status;
            entry.playtime_minutes = playtime_minutes;
            Ok(())
        })
    }

    fn add_achievement(
        &mut self,
        game_id: i64,
        achievement: NewAchievement,
    ) -> Result<i64, AppError> {
        let (name, description, points) = achievement.into_parts();
        self.update(|data| {
            let id = next_id(data.achievements.iter().map(|achievement| achievement.id))?;
            data.achievements.push(StoredAchievement {
                id,
                game_id,
                name,
                description,
                points,
            });
            Ok(id)
        })
    }

    fn achievements(&mut self, game_id: i64) -> Result<Vec<Achievement>, AppError> {
        if !self.data.games.iter().any(|game| game.id == game_id) {
            return Err(AppError::NotFound("Game not found. Use games to list IDs."));
        }
        let mut achievements: Vec<_> = self
            .data
            .achievements
            .iter()
            .filter(|achievement| achievement.game_id == game_id)
            .map(|achievement| Achievement {
                id: achievement.id,
                game_id: achievement.game_id,
                name: achievement.name.clone(),
                description: achievement.description.clone(),
                points: achievement.points,
            })
            .collect();
        achievements.sort_by_key(|achievement| achievement.id);
        Ok(achievements)
    }

    fn achievement(&mut self, achievement_id: i64) -> Result<Achievement, AppError> {
        let achievement = self
            .data
            .achievements
            .iter()
            .find(|achievement| achievement.id == achievement_id)
            .ok_or(AppError::NotFound(
                "Achievement not found. Use achievements to list IDs.",
            ))?;
        Ok(Achievement {
            id: achievement.id,
            game_id: achievement.game_id,
            name: achievement.name.clone(),
            description: achievement.description.clone(),
            points: achievement.points,
        })
    }

    fn unlock(&mut self, player_id: i64, achievement: Achievement) -> Result<(), AppError> {
        self.update(|data| {
            data.player_achievements.push(StoredPlayerAchievement {
                player_id,
                achievement_id: achievement.id,
                game_id: achievement.game_id,
                unlocked_at: Utc::now().to_rfc3339(),
            });
            Ok(())
        })
    }

    fn unlocked(&mut self, player_id: i64) -> Result<Vec<PlayerAchievement>, AppError> {
        self.require_player(player_id)?;
        let mut achievements: Vec<_> = self
            .data
            .player_achievements
            .iter()
            .filter(|unlock| unlock.player_id == player_id)
            .map(|unlock| {
                let achievement = self
                    .data
                    .achievements
                    .iter()
                    .find(|achievement| achievement.id == unlock.achievement_id)
                    .expect("validated achievement reference");
                let game = self
                    .data
                    .games
                    .iter()
                    .find(|game| game.id == unlock.game_id)
                    .expect("validated game reference");
                PlayerAchievement {
                    player_id: unlock.player_id,
                    achievement_id: unlock.achievement_id,
                    game_id: unlock.game_id,
                    game_title: game.title.clone(),
                    name: achievement.name.clone(),
                    points: achievement.points,
                    unlocked_at: unlock.unlocked_at.clone(),
                }
            })
            .collect();
        achievements.sort_by_key(|achievement| achievement.achievement_id);
        Ok(achievements)
    }
}

impl From<io::Error> for AppError {
    fn from(source: io::Error) -> Self {
        Self::Failure {
            message: "Could not access JSON storage. Check the file path, parent directory, permissions, and available disk space.".to_owned(),
            source: Box::new(source),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(source: serde_json::Error) -> Self {
        Self::Failure {
            message: "Invalid JSON library. Check the file contents.".to_owned(),
            source: Box::new(source),
        }
    }
}
