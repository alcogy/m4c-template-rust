use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    extract::Path,
};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Product {
    id: Option<i32>,
    name: String,
    price: i32,
    cost: i32,
    tax_rate: i16,
}

#[derive(Debug, Deserialize)]
pub struct ProductId {
    id: i32,
}

pub async fn find() -> impl IntoResponse {
    dotenv().ok();
    let database = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    let pool = PgPool::connect(database).await.expect("failed connect DB.");

    let products = sqlx::query_as!(Product, "SELECT * FROM product;")
        .fetch_all(&pool)
        .await
        .expect("Error SELECT");

    (StatusCode::OK, Json(products))
}

pub async fn find_by_id(Path(id): Path<i32>) -> impl IntoResponse {
    dotenv().ok();
    let database = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    let pool = PgPool::connect(database).await.expect("failed connect DB.");

    let product = sqlx::query_as!(Product, "SELECT * FROM product WHERE id = $1;", id)
        .fetch_one(&pool)
        .await
        .expect("Error SELECT");

    (StatusCode::OK, Json(product))
}

pub async fn create(Json(payload): Json<Product>) -> impl IntoResponse {
    dotenv().ok();
    let database = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    let pool = PgPool::connect(database).await.expect("failed connect DB.");

    sqlx::query!(r#"
        INSERT INTO product (id, name, price, cost, tax_rate)
        VALUES (default, $1, $2, $3, $4);"#,
        payload.name, payload.price, payload.cost, payload.tax_rate)
        .execute(&pool)
        .await
        .expect("Error Insert");

    (StatusCode::OK, "OK")
}

pub async fn update(Json(payload): Json<Product>) -> impl IntoResponse {
    dotenv().ok();
    let database = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    let pool = PgPool::connect(database).await.expect("failed connect DB.");

    sqlx::query!(r#"
        UPDATE product
        SET name = $1, price = $2, cost = $3, tax_rate = $4
        WHERE id = $5;"#,
        payload.name, payload.price, payload.cost, payload.tax_rate, payload.id)
        .execute(&pool)
        .await
        .expect("Error UPDATE");

    (StatusCode::OK, "OK")
}

pub async fn delete(Json(payload): Json<ProductId>) -> impl IntoResponse {
    dotenv().ok();
    let database = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    let pool = PgPool::connect(database).await.expect("failed connect DB.");

    sqlx::query!("DELETE FROM product WHERE id = $1;", payload.id)
        .execute(&pool)
        .await
        .expect("Error DELETE");

    (StatusCode::OK, "OK")
}