use std::fs;
use crate::model::Movie;

pub fn load_movies() -> Vec<Movie> {
    fs::read_to_string("movies.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_else(|_| vec![]))
        .unwrap_or_else(|_| vec![])
}

pub fn save_movies(movies: &[Movie]) {
    fs::write("movies.json", serde_json::to_string_pretty(movies).unwrap()).unwrap();
}