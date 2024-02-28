use sea_orm::{DatabaseConnection, SqlxMySqlConnector};
use sqlx::MySqlPool;

use crate::setting::MysqlConfig;

pub mod user;

pub async fn init_pool(mysql_config: MysqlConfig) -> MySqlPool {
    MySqlPool::connect(&format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql_config.user,
        mysql_config.password,
        mysql_config.host,
        mysql_config.port,
        mysql_config.db
    ))
    .await
    .expect("failed to connect database")
}

pub async fn init_conn(mysql_config: MysqlConfig) -> DatabaseConnection {
    let pool = init_pool(mysql_config).await;
    SqlxMySqlConnector::from_sqlx_mysql_pool(pool)
}
