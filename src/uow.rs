use sqlx::{PgConnection, PgPool};

pub struct UnitOfWork {
    conn: sqlx::pool::PoolConnection<sqlx::Postgres>,
}

impl UnitOfWork {
    pub async fn new(pool: &PgPool) -> Result<Self, sqlx::Error> {
        let conn = pool.acquire().await?;
        Ok(Self { conn }) 
    }

    pub fn conn_mut(&mut self) -> &mut PgConnection {
        &mut self.conn
    }
}

