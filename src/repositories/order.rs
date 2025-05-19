use crate::models::order::{Order, NewOrderItem};
use sqlx::PgConnection;

pub struct OrderRepository;

impl OrderRepository {
    pub async fn insert_order(
        conn: &mut PgConnection,
        order_date: chrono::NaiveDate,
        order_value: f64,
    ) -> Result<Order, sqlx::Error> {
        sqlx::query_as!(
            Order,
            "INSERT INTO orders (order_date, order_value) VALUES ($1, $2) RETURNING order_id, order_date, order_value",
            order_date,
            order_value
        )
        .fetch_one(conn)
        .await
    }

    pub async fn insert_order_items(
        conn: &mut PgConnection,
        order_id: i32,
        items: &[NewOrderItem],
    ) -> Result<(), sqlx::Error> {
        for item in items {
            sqlx::query!(
                "INSERT INTO order_items (order_id, product_id, quantity, total) VALUES ($1, $2, $3, $4)",
                order_id,
                item.product_id,
                item.quantity,
                item.quantity as f64 * item.unit_price
            )
            .execute(&mut *conn)
            .await?;
        }
        Ok(())
    }
}
