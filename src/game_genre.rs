#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameGenre {
    pub game_id: i64,
    pub genre_id: i64,
}
