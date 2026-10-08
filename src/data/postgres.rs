use postgres::{Client, NoTls, error::SqlState};

use crate::{
    app_error::AppError,
    domain::{
        Achievement, Game, GameGenre, Genre, Player, PlayerAchievement, PlayerGame, Status, Storage,
    },
};

pub struct PostgresStorage {
    db: Client,
}

impl PostgresStorage {
    pub fn connect(database_url: &str) -> Result<Self, AppError> {
        let db = Client::connect(database_url, NoTls).map_err(|source| AppError::Failure {
            message: "Could not connect to PostgreSQL. Check DATABASE_URL and start the database."
                .to_owned(),
            source: Box::new(source),
        })?;
        Ok(Self { db })
    }

    fn require_player(&mut self, id: i64) -> Result<(), AppError> {
        if self
            .db
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

impl Storage for PostgresStorage {
    fn add_player(&mut self, username: &str, email: &str) -> Result<i64, AppError> {
        Ok(self
            .db
            .query_one(
                "INSERT INTO player (username, email) VALUES ($1, $2) RETURNING id",
                &[&username, &email],
            )?
            .get(0))
    }

    fn players(&mut self) -> Result<Vec<Player>, AppError> {
        Ok(self
            .db
            .query("SELECT id, username, email FROM player ORDER BY id", &[])?
            .into_iter()
            .map(|row| Player {
                id: row.get(0),
                username: row.get(1),
                email: row.get(2),
            })
            .collect())
    }

    fn add_game(&mut self, title: &str, release_date: &str) -> Result<i64, AppError> {
        Game::validate_release_date(release_date)?;
        Ok(self
            .db
            .query_one(
                "INSERT INTO game (title, release_date) VALUES ($1, $2::text::date) RETURNING id",
                &[&title, &release_date],
            )?
            .get(0))
    }

    fn games(&mut self) -> Result<Vec<Game>, AppError> {
        Ok(self
            .db
            .query(
                "SELECT g.id, g.title, g.release_date::text,
                ARRAY(SELECT genre.name FROM game_genre gg JOIN genre ON genre.id = gg.genre_id
                      WHERE gg.game_id = g.id ORDER BY genre.name)
             FROM game g ORDER BY g.id",
                &[],
            )?
            .into_iter()
            .map(|row| Game {
                id: row.get(0),
                title: row.get(1),
                release_date: row.get(2),
                genres: row.get(3),
            })
            .collect())
    }

    fn add_genre(&mut self, name: &str) -> Result<i64, AppError> {
        Ok(self
            .db
            .query_one(
                "INSERT INTO genre (name) VALUES ($1) RETURNING id",
                &[&name],
            )?
            .get(0))
    }

    fn genres(&mut self) -> Result<Vec<Genre>, AppError> {
        Ok(self
            .db
            .query("SELECT id, name FROM genre ORDER BY id", &[])?
            .into_iter()
            .map(|row| Genre {
                id: row.get(0),
                name: row.get(1),
            })
            .collect())
    }

    fn tag_game(&mut self, tag: GameGenre) -> Result<(), AppError> {
        self.db.execute(
            "INSERT INTO game_genre (game_id, genre_id) VALUES ($1, $2)",
            &[&tag.game_id, &tag.genre_id],
        )?;
        Ok(())
    }

    fn add_to_library(&mut self, player_id: i64, game_id: i64) -> Result<(), AppError> {
        self.db.execute(
            "INSERT INTO player_game (player_id, game_id) VALUES ($1, $2)",
            &[&player_id, &game_id],
        )?;
        Ok(())
    }

    fn library(&mut self, player_id: i64) -> Result<Vec<PlayerGame>, AppError> {
        self.require_player(player_id)?;
        Ok(self.db
            .query(
                "SELECT g.id, g.title, pg.status, pg.playtime_minutes, pg.player_id, pg.added_at::text
             FROM player_game pg JOIN game g ON g.id = pg.game_id
             WHERE pg.player_id = $1 ORDER BY g.id",
                &[&player_id],
            )?
            .into_iter()
            .map(|row| PlayerGame {
                game_id: row.get(0),
                title: row.get(1),
                status: row.get(2),
                playtime_minutes: row.get(3),
                player_id: row.get(4),
                added_at: row.get(5),
            })
            .collect())
    }

    fn set_progress(
        &mut self,
        player_id: i64,
        game_id: i64,
        status: Status,
        playtime_minutes: i32,
    ) -> Result<(), AppError> {
        let status = status.to_string();
        if self.db.execute(
            "UPDATE player_game SET status = $3, playtime_minutes = $4 WHERE player_id = $1 AND game_id = $2",
            &[&player_id, &game_id, &status, &playtime_minutes],
        )? == 0 {
            return Err(AppError::NotFound("Game not found in this player's library. Check the player and game IDs, or use add-to-library."));
        }
        Ok(())
    }

    fn add_achievement(
        &mut self,
        game_id: i64,
        name: &str,
        description: &str,
        points: i32,
    ) -> Result<i64, AppError> {
        Ok(self.db.query_one(
            "INSERT INTO achievement (game_id, name, description, points) VALUES ($1, $2, $3, $4) RETURNING id",
            &[&game_id, &name, &description, &points],
        )?.get(0))
    }

    fn achievements(&mut self, game_id: i64) -> Result<Vec<Achievement>, AppError> {
        if self
            .db
            .query_opt("SELECT id FROM game WHERE id = $1", &[&game_id])?
            .is_none()
        {
            return Err(AppError::NotFound("Game not found. Use games to list IDs."));
        }
        Ok(self.db.query(
            "SELECT id, name, description, points, game_id FROM achievement WHERE game_id = $1 ORDER BY id", &[&game_id],
        )?.into_iter().map(|row| Achievement {
            id: row.get(0), name: row.get(1), description: row.get(2), points: row.get(3), game_id: row.get(4),
        }).collect())
    }

    fn unlock(&mut self, player_id: i64, achievement_id: i64) -> Result<(), AppError> {
        if self.db.execute(
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

    fn unlocked(&mut self, player_id: i64) -> Result<Vec<PlayerAchievement>, AppError> {
        self.require_player(player_id)?;
        Ok(self.db
            .query(
                "SELECT a.id, g.title, a.name, a.points, pa.unlocked_at::text, pa.player_id, pa.game_id
             FROM player_achievement pa JOIN achievement a ON a.id = pa.achievement_id
             JOIN game g ON g.id = a.game_id WHERE pa.player_id = $1 ORDER BY a.id",
                &[&player_id],
            )?
            .into_iter()
            .map(|row| PlayerAchievement {
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

// Convert SQL diagnostics here so commands and other backends never depend on PostgreSQL errors.
impl From<postgres::Error> for AppError {
    fn from(source: postgres::Error) -> Self {
        let error = match source.code() {
            Some(&SqlState::UNIQUE_VIOLATION) => AppError::AlreadyExists,
            Some(&SqlState::FOREIGN_KEY_VIOLATION) => AppError::InvalidReference,
            Some(&SqlState::CHECK_VIOLATION | &SqlState::NOT_NULL_VIOLATION) => {
                AppError::InvalidValue
            }
            Some(&SqlState::INVALID_DATETIME_FORMAT | &SqlState::DATETIME_FIELD_OVERFLOW) => {
                AppError::InvalidDate
            }
            Some(&SqlState::UNDEFINED_TABLE) => AppError::NotFound(
                "The database schema is missing. Initialize the database using schema.sql.",
            ),
            _ => AppError::NotFound(
                "Database operation failed. Check the database connection and server logs.",
            ),
        };
        AppError::Failure {
            message: error.to_string(),
            source: Box::new(source),
        }
    }
}
