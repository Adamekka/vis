use postgres::Client;

use crate::app_error::AppError;

pub struct GameGenre {
    pub game_id: i64,
    pub genre_id: i64,
}

impl GameGenre {
    pub fn add(&self, db: &mut Client) -> Result<(), AppError> {
        db.execute(
            "INSERT INTO game_genre (game_id, genre_id) VALUES ($1, $2)",
            &[&self.game_id, &self.genre_id],
        )?;
        Ok(())
    }
}
