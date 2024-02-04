use sea_orm::{DatabaseConnection, SqlxMySqlConnector};
use sqlx::MySqlPool;
use tokio::sync::OnceCell;

pub mod user;
static POOL: OnceCell<MySqlPool> = OnceCell::const_new();

pub async fn init_pool() -> MySqlPool {
    MySqlPool::connect("mysql://root:613181hyy@localhost:3306/axum")
        .await
        .expect("failed to connect database")
}

pub async fn get_pool() -> &'static MySqlPool {
    POOL.get_or_init(init_pool).await
}

pub async fn init_conn() -> DatabaseConnection {
    let pool = init_pool().await;
    SqlxMySqlConnector::from_sqlx_mysql_pool(pool)
}

static CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_conn() -> &'static DatabaseConnection {
    CONN.get_or_init(init_conn).await
}
