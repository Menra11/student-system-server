use mysql::*;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    pool: Arc<Pool>,
}
impl Database {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let host = env::var("MYSQL_HOST")?;
        let port = env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_string());
        let user = env::var("MYSQL_USER")?;
        let password = env::var("MYSQL_PASSWORD")?;
        let database = env::var("MYSQL_DATABASE")?;

        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );

        let opts = Opts::from_url(&url)?;
        let pool = Pool::new(opts)?;
        println!("✅ 数据库连接成功");

        Ok(Self { pool: Arc::new(pool) })
    }

    pub async fn get_connection(&self) -> Result<PooledConn, Box<dyn std::error::Error>> {
        Ok(self.pool.get_conn()?)
    }
}