use postgres::Client;

use crate::app_error::AppError;

pub struct Game {
    pub id: i64,
    pub title: String,
    pub release_date: String,
    pub genres: Vec<String>,
}

impl Game {
    pub fn add(db: &mut Client, title: &str, release_date: &str) -> Result<i64, AppError> {
        if release_date.len() != 10
            || !release_date.bytes().enumerate().all(|(index, byte)| {
                if index == 4 || index == 7 {
                    byte == b'-'
                } else {
                    byte.is_ascii_digit()
                }
            })
        {
            return Err(AppError::InvalidDate);
        }
        Ok(db
            .query_one(
                "INSERT INTO game (title, release_date) VALUES ($1, $2::text::date) RETURNING id",
                &[&title, &release_date],
            )?
            .get(0))
    }

    pub fn list(db: &mut Client) -> Result<Vec<Self>, AppError> {
        Ok(db
            .query(
                "SELECT g.id, g.title, g.release_date::text,
                ARRAY(SELECT genre.name FROM game_genre gg JOIN genre ON genre.id = gg.genre_id
                      WHERE gg.game_id = g.id ORDER BY genre.name)
             FROM game g ORDER BY g.id",
                &[],
            )?
            .into_iter()
            .map(|row| Self {
                id: row.get(0),
                title: row.get(1),
                release_date: row.get(2),
                genres: row.get(3),
            })
            .collect())
    }
}
