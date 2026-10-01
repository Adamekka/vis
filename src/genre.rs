#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}
