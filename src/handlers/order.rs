use sqlx::PgPool;
use axum::extract::{Path, State};
use axum::Json;
use crate::models::order::{NewOrder, Order, OrderItem, OrderWithItems};
use hyper::StatusCode;
use crate::uow::UnitOfWork;
use crate::services::order_service::OrderService;
pub async fn create_order(
    State(pool): State<PgPool>,
    Json(payload): Json<NewOrder>,
) -> Result<StatusCode, StatusCode> {
    let mut uow = UnitOfWork::new(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    OrderService::create_order(&mut uow, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn get_order(
    Path(order_id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<OrderWithItems>, StatusCode> {
    let order = sqlx::query_as::<_, Order>("SELECT order_id, order_date, order_value FROM orders WHERE order_id = $1")
        .bind(order_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let items = sqlx::query_as::<_, OrderItem>(
        r#"SELECT oi.order_item_id, oi.order_id, oi.product_id, oi.quantity,
           (oi.quantity * p.unit_price) as total
           FROM order_items oi
           JOIN products p ON oi.product_id = p.product_id
           WHERE oi.order_id = $1"#
    )
    .bind(order_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(OrderWithItems { order, items }))
}

pub async fn list_orders(
    State(pool): State<PgPool>
) -> Result<Json<Vec<Order>>, StatusCode> {
    let orders = sqlx::query_as::<_, Order>(
        "SELECT order_id, order_date, order_value FROM orders ORDER BY order_id DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(orders))
}
