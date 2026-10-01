#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub id: i64,
    pub username: String,
    pub email: String,
}
