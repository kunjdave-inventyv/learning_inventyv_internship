use axum::{extract::State, response::Json};
use uuid::Uuid;
use crate::{SharedState, handler::save_movies, model::{ApiRequest, Movie}};
use crate::executor::Executor;

#[derive(Clone)]
pub struct Pools {
    pub read: Executor,
    pub write: Executor,
}

pub async fn dispatch(
    State((state, pools)): State<(SharedState, Pools)>,
    Json(req): Json<ApiRequest>,
) -> Json<serde_json::Value> {

    match req.action.as_str() {
        "get" => {
            pools.read.run(async move {
                let movies = state.read().await;
                serde_json::json!(movies.clone())
            }).await
        }

        "add" => {
            pools.write.run(async move {
                let mut movies = state.write().await;
                let mut movie = req.movie.unwrap();
                movie.id = Uuid::new_v4().to_string();
                movies.push(movie);
                save_movies(&movies);
                serde_json::json!({"status":"created"})
            }).await
        }

        "update" => {
            pools.write.run(async move {
                let mut movies = state.write().await;
                let id = req.id.unwrap();
                let new = req.movie.unwrap();

                if let Some(m) = movies.iter_mut().find(|m| m.id == id) {
                    m.title = new.title;
                    m.director = new.director;
                    m.year = new.year;
                    save_movies(&movies);
                    serde_json::json!({"status":"updated"})
                } else {
                    serde_json::json!({"error":"not found"})
                }
            }).await
        }

        "delete" => {
            pools.write.run(async move {
                let mut movies = state.write().await;
                let id = req.id.unwrap();
                movies.retain(|m| m.id != id);
                save_movies(&movies);
                serde_json::json!({"status":"deleted"})
            }).await
        }

        _ => serde_json::json!({"error":"invalid action"}),
    }.into()
}
