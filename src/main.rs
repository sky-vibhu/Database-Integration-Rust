mod db;
mod handlers;
mod models;
mod repositories;
mod services;
mod uow;

use axum::{routing::{get, post}, Router};
use db::init_pool;
use handlers::order::{create_order, get_order, list_orders};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = init_pool().await;

    let app = Router::new()
        .route("/create_order", post(create_order))
        .route("/get_order/:{id}", get(get_order))
        .route("/orders", get(list_orders))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
