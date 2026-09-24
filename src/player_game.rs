use postgres::Client;

use crate::{app_error::AppError, player::Player, status::Status};

pub struct PlayerGame {
    pub player_id: i64,
    pub game_id: i64,
    pub title: String,
    pub status: String,
    pub playtime_minutes: i32,
    pub added_at: String,
}

impl PlayerGame {
    pub fn add(db: &mut Client, player_id: i64, game_id: i64) -> Result<(), AppError> {
        db.execute(
            "INSERT INTO player_game (player_id, game_id) VALUES ($1, $2)",
            &[&player_id, &game_id],
        )?;
        Ok(())
    }

    pub fn list(db: &mut Client, player_id: i64) -> Result<Vec<Self>, AppError> {
        Player::require(db, player_id)?;
        Ok(db
            .query(
                "SELECT g.id, g.title, pg.status, pg.playtime_minutes, pg.player_id, pg.added_at::text
             FROM player_game pg JOIN game g ON g.id = pg.game_id
             WHERE pg.player_id = $1 ORDER BY g.id",
                &[&player_id],
            )?
            .into_iter()
            .map(|row| Self {
                game_id: row.get(0),
                title: row.get(1),
                status: row.get(2),
                playtime_minutes: row.get(3),
                player_id: row.get(4),
                added_at: row.get(5),
            })
            .collect())
    }

    pub fn set_progress(
        db: &mut Client,
        player_id: i64,
        game_id: i64,
        status: Status,
        playtime_minutes: i32,
    ) -> Result<(), AppError> {
        let status = match status {
            Status::NotStarted => "not_started",
            Status::Playing => "playing",
            Status::Completed => "completed",
            Status::Abandoned => "abandoned",
        };
        if db.execute(
            "UPDATE player_game SET status = $3, playtime_minutes = $4 WHERE player_id = $1 AND game_id = $2",
            &[&player_id, &game_id, &status, &playtime_minutes],
        )? == 0 {
            return Err(AppError::NotFound("Game not found in this player's library. Check the player and game IDs, or use add-to-library."));
        }
        Ok(())
    }
}
