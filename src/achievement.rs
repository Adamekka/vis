use postgres::Client;

use crate::app_error::AppError;

pub struct Achievement {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub description: String,
    pub points: i32,
}

impl Achievement {
    pub fn add(
        db: &mut Client,
        game_id: i64,
        name: &str,
        description: &str,
        points: i32,
    ) -> Result<i64, AppError> {
        Ok(db.query_one(
            "INSERT INTO achievement (game_id, name, description, points) VALUES ($1, $2, $3, $4) RETURNING id",
            &[&game_id, &name, &description, &points],
        )?.get(0))
    }

    pub fn list(db: &mut Client, game_id: i64) -> Result<Vec<Self>, AppError> {
        if db
            .query_opt("SELECT id FROM game WHERE id = $1", &[&game_id])?
            .is_none()
        {
            return Err(AppError::NotFound("Game not found. Use games to list IDs."));
        }
        Ok(db.query(
            "SELECT id, name, description, points, game_id FROM achievement WHERE game_id = $1 ORDER BY id", &[&game_id],
        )?.into_iter().map(|row| Self {
            id: row.get(0), name: row.get(1), description: row.get(2), points: row.get(3), game_id: row.get(4),
        }).collect())
    }
}
