use super::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub enum ProjectState {
    #[serde(rename = "not_created")]
    NotCreated,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "installed")]
    Installed
}