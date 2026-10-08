mod achievement;
mod all_items;
mod game;
mod game_genre;
mod genre;
mod player;
mod player_achievement;
mod player_game;
mod request;
mod status;
mod storage;

pub use achievement::{Achievement, NewAchievement};
pub use all_items::AllItems;
pub use game::{Game, NewGame};
pub use game_genre::GameGenre;
pub use genre::{Genre, NewGenre};
pub use player::{NewPlayer, Player};
pub use player_achievement::PlayerAchievement;
pub use player_game::{PlayerGame, Progress};
pub use request::{Request, Response};
pub use status::Status;
pub use storage::Storage;

#[cfg(test)]
mod tests {
    use crate::app_error::AppError;

    use super::{NewAchievement, NewGame, NewGenre, NewPlayer, Progress, Status};

    #[test]
    fn blank_text_is_rejected_without_cli_parsing() {
        for blank in ["", " \t\n", "\u{2003}"] {
            assert!(matches!(
                NewPlayer::new(blank.to_owned(), "alex@example.test".to_owned()),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewPlayer::new("alex".to_owned(), blank.to_owned()),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewGame::new(blank.to_owned(), "2024-02-29".to_owned()),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewGenre::new(blank.to_owned()),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewAchievement::new(blank.to_owned(), "Finish the tutorial".to_owned(), 0),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewAchievement::new("First step".to_owned(), blank.to_owned(), 0),
                Err(AppError::InvalidValue)
            ));
        }
    }

    #[test]
    fn release_dates_are_valid_calendar_dates_in_the_supported_format() {
        for date in [
            "2025-02-29",
            "2025-02-30",
            "0000-01-01",
            "2024-2-29",
            "tomorrow",
        ] {
            assert!(matches!(
                NewGame::new("Game".to_owned(), date.to_owned()),
                Err(AppError::InvalidDate)
            ));
        }
        assert!(NewGame::new("Game".to_owned(), "2024-02-29".to_owned()).is_ok());
    }

    #[test]
    fn negative_points_and_playtime_are_rejected_without_cli_parsing() {
        for value in [i32::MIN, -1] {
            assert!(matches!(
                Progress::new(Status::Playing, value),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewAchievement::new(
                    "First step".to_owned(),
                    "Finish the tutorial".to_owned(),
                    value
                ),
                Err(AppError::InvalidValue)
            ));
        }
        for value in [0, i32::MAX] {
            assert!(Progress::new(Status::Playing, value).is_ok());
            assert!(
                NewAchievement::new(
                    "First step".to_owned(),
                    "Finish the tutorial".to_owned(),
                    value
                )
                .is_ok()
            );
        }
    }
}
