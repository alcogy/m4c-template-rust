mod handlers;

use handlers::{get_handler, get_by_id_handler, post_handler, put_handler, delete_handler};
use axum::{
    routing::{get},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {    
    // Web Server
    let app = Router::new()
        .route("/", get(get_handler).post(post_handler).put(put_handler).delete(delete_handler))
        .route("/:id", get(get_by_id_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
