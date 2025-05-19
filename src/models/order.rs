use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Serialize, FromRow)]
pub struct Order {
    pub order_id: i32,
    pub order_date: NaiveDate,
    pub order_value: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrderItem {
    pub order_item_id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub total: f64,
}

#[derive(Debug, Serialize)]
pub struct OrderWithItems {
    pub order: Order,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Deserialize)]
pub struct NewOrderItem {
    pub product_id: i32,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Debug, Deserialize)]
pub struct NewOrder {
    pub order_date: NaiveDate,
    pub items: Vec<NewOrderItem>,
}
