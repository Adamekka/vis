use postgres::Client;

use crate::{app_error::AppError, player::Player};

pub struct PlayerAchievement {
    pub player_id: i64,
    pub achievement_id: i64,
    pub game_id: i64,
    pub game_title: String,
    pub name: String,
    pub points: i32,
    pub unlocked_at: String,
}

impl PlayerAchievement {
    pub fn unlock(db: &mut Client, player_id: i64, achievement_id: i64) -> Result<(), AppError> {
        if db.execute(
            "INSERT INTO player_achievement (player_id, achievement_id, game_id)
             SELECT $1, id, game_id FROM achievement WHERE id = $2",
            &[&player_id, &achievement_id],
        )? == 0
        {
            return Err(AppError::NotFound(
                "Achievement not found. Use achievements to list IDs.",
            ));
        }
        Ok(())
    }

    pub fn list(db: &mut Client, player_id: i64) -> Result<Vec<Self>, AppError> {
        Player::require(db, player_id)?;
        Ok(db
            .query(
                "SELECT a.id, g.title, a.name, a.points, pa.unlocked_at::text, pa.player_id, pa.game_id
             FROM player_achievement pa JOIN achievement a ON a.id = pa.achievement_id
             JOIN game g ON g.id = a.game_id WHERE pa.player_id = $1 ORDER BY a.id",
                &[&player_id],
            )?
            .into_iter()
            .map(|row| Self {
                achievement_id: row.get(0),
                game_title: row.get(1),
                name: row.get(2),
                points: row.get(3),
                unlocked_at: row.get(4),
                player_id: row.get(5),
                game_id: row.get(6),
            })
            .collect())
    }
}
