use postgres::Client;

use crate::app_error::AppError;

pub struct Genre {
    pub id: i64,
    pub name: String,
}

impl Genre {
    pub fn add(db: &mut Client, name: &str) -> Result<i64, AppError> {
        Ok(db
            .query_one(
                "INSERT INTO genre (name) VALUES ($1) RETURNING id",
                &[&name],
            )?
            .get(0))
    }

    pub fn list(db: &mut Client) -> Result<Vec<Self>, AppError> {
        Ok(db
            .query("SELECT id, name FROM genre ORDER BY id", &[])?
            .into_iter()
            .map(|row| Self {
                id: row.get(0),
                name: row.get(1),
            })
            .collect())
    }
}
