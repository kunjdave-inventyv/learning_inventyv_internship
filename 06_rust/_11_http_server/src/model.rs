use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Movie {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub director: String,
    pub year: u16,
}

#[derive(Deserialize)]
pub struct ApiRequest {
    pub action: String,        // "get", "add", "update", "delete"
    pub id: Option<String>,
    pub movie: Option<Movie>,
}
