use tokio::runtime::Builder;
use std::{sync::Arc, net::SocketAddr};
use tokio::{net::TcpListener, sync::RwLock};

mod model;
mod handler;
mod api;
mod routes;
mod executor;

use model::Movie;
use handler::load_movies;
use routes::movie_routes;
use api::Pools;
use executor::Executor;

pub type SharedState = Arc<RwLock<Vec<Movie>>>;

#[tokio::main(flavor="multi_thread")]
async fn main() {
    let movies = load_movies();
    let state: SharedState = Arc::new(RwLock::new(movies));

    let read_rt = Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    let write_rt = Builder::new_multi_thread().worker_threads(2).enable_all().build().unwrap();

    let pools = Pools {
        read: Executor::new(read_rt),
        write: Executor::new(write_rt),
    };

    let app = movie_routes(state, pools);

    let addr = SocketAddr::from(([127,0,0,1], 4500));
    println!("Server running at http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
