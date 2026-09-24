use postgres::Client;

use crate::app_error::AppError;

pub struct Player {
    pub id: i64,
    pub username: String,
    pub email: String,
}

impl Player {
    pub fn add(db: &mut Client, username: &str, email: &str) -> Result<i64, AppError> {
        Ok(db
            .query_one(
                "INSERT INTO player (username, email) VALUES ($1, $2) RETURNING id",
                &[&username, &email],
            )?
            .get(0))
    }

    pub fn list(db: &mut Client) -> Result<Vec<Self>, AppError> {
        Ok(db
            .query("SELECT id, username, email FROM player ORDER BY id", &[])?
            .into_iter()
            .map(|row| Self {
                id: row.get(0),
                username: row.get(1),
                email: row.get(2),
            })
            .collect())
    }

    pub fn require(db: &mut Client, id: i64) -> Result<(), AppError> {
        if db
            .query_opt("SELECT id FROM player WHERE id = $1", &[&id])?
            .is_none()
        {
            return Err(AppError::NotFound(
                "Player not found. Use players to list IDs.",
            ));
        }
        Ok(())
    }
}
