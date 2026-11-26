mod achievement;
mod action;
mod all_items;
mod authentication;
mod authorization;
mod current_user;
mod game;
mod game_genre;
mod genre;
mod library_achievement;
mod mapper;
mod operation;
mod player;
mod player_achievement;
mod player_game;
mod player_library;
mod player_listing;
mod role;
mod status;
mod storage;
mod user;

pub use achievement::{Achievement, NewAchievement};
pub use action::Action;
pub use all_items::AllItems;
pub use authentication::authenticate;
pub use current_user::CurrentUser;
pub use game::{Game, NewGame};
pub use game_genre::GameGenre;
pub use genre::{Genre, NewGenre};
pub use library_achievement::LibraryAchievement;
pub use operation::{Operation, Outcome};
pub use player::{NewPlayer, Player};
pub use player_achievement::PlayerAchievement;
pub use player_game::{PlayerGame, Progress};
pub use player_library::PlayerLibrary;
pub use player_listing::{Email, PlayerListing};
pub use role::Role;
pub use status::Status;
pub use storage::Storage;
pub use user::User;

#[cfg(test)]
mod tests {
    use crate::app_error::AppError;

    use super::{NewAchievement, NewGame, NewGenre, NewPlayer, Progress, Status};

    #[test]
    fn blank_text_is_rejected_without_cli_parsing() {
        for blank in ["", " \t\n", "\u{2003}"] {
            assert!(matches!(
                NewPlayer::try_from((blank.to_owned(), "alex@example.test".to_owned())),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewPlayer::try_from(("alex".to_owned(), blank.to_owned())),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewGame::try_from((blank.to_owned(), "2024-02-29".to_owned())),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewGenre::try_from(blank.to_owned()),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewAchievement::try_from((blank.to_owned(), "Finish the tutorial".to_owned(), 0)),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewAchievement::try_from(("First step".to_owned(), blank.to_owned(), 0)),
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
                NewGame::try_from(("Game".to_owned(), date.to_owned())),
                Err(AppError::InvalidDate)
            ));
        }
        assert!(NewGame::try_from(("Game".to_owned(), "2024-02-29".to_owned())).is_ok());
    }

    #[test]
    fn negative_points_and_playtime_are_rejected_without_cli_parsing() {
        for value in [i32::MIN, -1] {
            assert!(matches!(
                Progress::try_from((Status::Playing, value)),
                Err(AppError::InvalidValue)
            ));
            assert!(matches!(
                NewAchievement::try_from((
                    "First step".to_owned(),
                    "Finish the tutorial".to_owned(),
                    value
                )),
                Err(AppError::InvalidValue)
            ));
        }
        for value in [0, i32::MAX] {
            assert!(Progress::try_from((Status::Playing, value)).is_ok());
            assert!(
                NewAchievement::try_from((
                    "First step".to_owned(),
                    "Finish the tutorial".to_owned(),
                    value
                ))
                .is_ok()
            );
        }
    }
}
