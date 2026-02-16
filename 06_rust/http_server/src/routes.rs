use axum::{Router, routing::post};
use crate::api::dispatch;
use crate::{SharedState, api::Pools};

pub fn movie_routes(state: SharedState, pools: Pools) -> Router {
    Router::new()
        .route("/movies", post(dispatch))
        .with_state((state, pools))
}
