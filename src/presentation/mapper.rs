//! Maps CLI commands to service requests and service results to display models.

use crate::service::models::{self as service, Request};

use super::{command::Command, models, status::Status};

impl From<Command> for Request {
    fn from(command: Command) -> Self {
        match command {
            Command::AllItems => Request::AllItems,
            Command::AddPlayer { username, email } => Request::AddPlayer { username, email },
            Command::Players => Request::Players,
            Command::AddGame {
                title,
                release_date,
            } => Request::AddGame {
                title,
                release_date,
            },
            Command::Games => Request::Games,
            Command::AddGenre { name } => Request::AddGenre { name },
            Command::Genres => Request::Genres,
            Command::TagGame { game_id, genre_id } => Request::TagGame { game_id, genre_id },
            Command::AddToLibrary { player_id, game_id } => {
                Request::AddToLibrary { player_id, game_id }
            }
            Command::Library { player_id } => Request::Library { player_id },
            Command::SetProgress {
                player_id,
                game_id,
                status,
                playtime_minutes,
            } => Request::SetProgress {
                player_id,
                game_id,
                status: status.into(),
                playtime_minutes,
            },
            Command::AddAchievement {
                game_id,
                name,
                description,
                points,
            } => Request::AddAchievement {
                game_id,
                name,
                description,
                points,
            },
            Command::Achievements { game_id } => Request::Achievements { game_id },
            Command::Unlock {
                player_id,
                achievement_id,
            } => Request::Unlock {
                player_id,
                achievement_id,
            },
            Command::Unlocked { player_id } => Request::Unlocked { player_id },
        }
    }
}

impl From<Status> for service::Status {
    fn from(status: Status) -> Self {
        match status {
            Status::NotStarted => Self::NotStarted,
            Status::Playing => Self::Playing,
            Status::Completed => Self::Completed,
            Status::Abandoned => Self::Abandoned,
        }
    }
}

impl From<service::Player> for models::Player {
    fn from(value: service::Player) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
        }
    }
}

impl From<service::Game> for models::Game {
    fn from(value: service::Game) -> Self {
        Self {
            id: value.id,
            title: value.title,
            release_date: value.release_date,
            genres: value.genres.join(", "),
        }
    }
}

impl From<service::Genre> for models::Genre {
    fn from(value: service::Genre) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

impl From<service::GameGenre> for models::GameGenre {
    fn from(value: service::GameGenre) -> Self {
        Self {
            game_id: value.game_id,
            genre_id: value.genre_id,
        }
    }
}

impl From<service::PlayerGame> for models::PlayerGame {
    fn from(value: service::PlayerGame) -> Self {
        Self {
            player_id: value.player_id,
            game_id: value.game_id,
            title: value.title,
            status: match value.status {
                service::Status::NotStarted => "not_started",
                service::Status::Playing => "playing",
                service::Status::Completed => "completed",
                service::Status::Abandoned => "abandoned",
            }
            .to_owned(),
            playtime_minutes: value.playtime_minutes,
            added_at: value.added_at,
        }
    }
}

impl From<service::Achievement> for models::Achievement {
    fn from(value: service::Achievement) -> Self {
        Self {
            id: value.id,
            game_id: value.game_id,
            name: value.name,
            description: value.description,
            points: value.points,
        }
    }
}

impl From<service::PlayerAchievement> for models::PlayerAchievement {
    fn from(value: service::PlayerAchievement) -> Self {
        Self {
            player_id: value.player_id,
            achievement_id: value.achievement_id,
            game_id: value.game_id,
            game_title: value.game_title,
            name: value.name,
            points: value.points,
            unlocked_at: value.unlocked_at,
        }
    }
}

impl From<service::AllItems> for models::AllItems {
    fn from(value: service::AllItems) -> Self {
        Self {
            players: value.players.into_iter().map(Into::into).collect(),
            games: value.games.into_iter().map(Into::into).collect(),
            genres: value.genres.into_iter().map(Into::into).collect(),
            game_genres: value.game_genres.into_iter().map(Into::into).collect(),
            library: value.library.into_iter().map(Into::into).collect(),
            achievements: value.achievements.into_iter().map(Into::into).collect(),
            unlocked: value.unlocked.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<service::Response> for models::Response {
    fn from(response: service::Response) -> Self {
        match response {
            service::Response::AllItems(value) => Self::AllItems(value.into()),
            service::Response::Created(value) => Self::Created(value),
            service::Response::Players(value) => {
                Self::Players(value.into_iter().map(Into::into).collect())
            }
            service::Response::Games(value) => {
                Self::Games(value.into_iter().map(Into::into).collect())
            }
            service::Response::Genres(value) => {
                Self::Genres(value.into_iter().map(Into::into).collect())
            }
            service::Response::GenreAssigned => Self::GenreAssigned,
            service::Response::AddedToLibrary => Self::AddedToLibrary,
            service::Response::Library(value) => {
                Self::Library(value.into_iter().map(Into::into).collect())
            }
            service::Response::ProgressUpdated => Self::ProgressUpdated,
            service::Response::Achievements(value) => {
                Self::Achievements(value.into_iter().map(Into::into).collect())
            }
            service::Response::AchievementUnlocked => Self::AchievementUnlocked,
            service::Response::Unlocked(value) => {
                Self::Unlocked(value.into_iter().map(Into::into).collect())
            }
        }
    }
}
