mod handlers;

use handlers::{find, find_by_id, create, update, delete};
use axum::{
    routing::{get},
    Router,
};
use std::net::SocketAddr;
use hyper::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {    
    // Web Server
    let app = Router::new()
        .route("/", get(find).post(create).put(update).delete(delete))
        .route("/:id", get(find_by_id))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(vec![CONTENT_TYPE]));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
