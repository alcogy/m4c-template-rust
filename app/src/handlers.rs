use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    extract::Path,
};
use serde::Serialize;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

#[derive(sqlx::FromRow, Debug, Serialize)]
struct Product {
    id: i32,
    name: String,
    price: i32,
    cost: i32,
    tax_rate: i16,
}

pub async fn get_handler() -> impl IntoResponse {
    dotenv().ok();
    let database = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    let pool = PgPool::connect(database).await.expect("failed connect DB.");

    let products = sqlx::query_as!(Product, "SELECT * FROM product;")
        .fetch_all(&pool)
        .await
        .expect("Error SELECT");

    (StatusCode::OK, Json(products))
}

pub async fn get_by_id_handler(Path(id): Path<i32>) -> impl IntoResponse {
    dotenv().ok();
    let database = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    let pool = PgPool::connect(database).await.expect("failed connect DB.");

    let product = sqlx::query_as!(Product, "SELECT * FROM product WHERE id = $1;", id)
        .fetch_one(&pool)
        .await
        .expect("Error SELECT");

    (StatusCode::OK, Json(product))
}

pub async fn post_handler() -> impl IntoResponse {
    (StatusCode::OK, "Hello POST")
}

pub async fn put_handler() -> impl IntoResponse {
    (StatusCode::OK, "Hello Put")
}

pub async fn delete_handler() -> impl IntoResponse {
    (StatusCode::OK, "Hello Delete")
}