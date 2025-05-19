use crate::models::order::NewOrder;
use crate::repositories::order::OrderRepository;
use crate::uow::UnitOfWork;

pub struct OrderService;

impl OrderService {
    pub async fn create_order(
        uow: &mut UnitOfWork,
        order: NewOrder,
    ) -> Result<(), sqlx::Error> {
        let total: f64 = order.items.iter()
            .map(|item| item.unit_price * item.quantity as f64)
            .sum();

        let inserted = OrderRepository::insert_order(
            uow.conn_mut(),
            order.order_date,
            total
        ).await?;

        OrderRepository::insert_order_items(
            uow.conn_mut(),
            inserted.order_id,
            &order.items
        ).await?;

        Ok(())
    }
}
