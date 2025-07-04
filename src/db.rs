use sqlx::MySqlPool;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    pool: Arc<MySqlPool>,  // 使用 sqlx 的 MySqlPool
}

impl Database {
    // 改为异步构造函数
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let host = env::var("MYSQL_HOST")?;
        let port = env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_string());
        let user = env::var("MYSQL_USER")?;
        let password = env::var("MYSQL_PASSWORD")?;
        let database = env::var("MYSQL_DATABASE")?;

        // 直接使用标准 URL 格式
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );

        // 使用 sqlx 的连接池构建器
        let pool = MySqlPool::connect(&url).await?;
        println!("✅ 数据库连接成功");

        Ok(Self { pool: Arc::new(pool) })
    }

    // 返回类型改为 sqlx 的连接类型
    pub async fn get_connection(&self) -> Result<sqlx::pool::PoolConnection<sqlx::MySql>, Box<dyn std::error::Error>> {
        // 直接获取连接（异步操作）
        Ok(self.pool.acquire().await?)
    }
}