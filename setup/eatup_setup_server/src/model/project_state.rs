use super::Serialize;

#[derive(Debug, Serialize)]
pub enum ProjectState {
    #[serde(rename = "not_created")]
    NotCreated,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "installed")]
    Installed
}