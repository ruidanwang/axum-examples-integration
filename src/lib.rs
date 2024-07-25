pub mod user;
pub mod org;
pub mod comm;
pub mod dbsqlx;
pub mod dbdiesel;
pub mod metrics;
pub mod rudis;
pub mod notifications;

use sqlx::postgres::PgPoolOptions;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::Duration;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use bb8_redis::bb8;

pub fn establish_connection() -> PgConnection {
    let diesel_db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:wangc4@localhost".to_string());
    PgConnection::establish(&diesel_db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", diesel_db_url))
}

pub fn establish_diesel_pool() -> deadpool_diesel::Pool<deadpool_diesel::Manager<PgConnection>>{
    let diesel_db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:wangc4@localhost".to_string());
    let manager = deadpool_diesel::postgres::Manager::new(diesel_db_url, deadpool_diesel::Runtime::Tokio1);
    deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap()
}

pub async  fn establish_redis_conn_pool()->Pool<RedisConnectionManager>{
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    bb8::Pool::builder().build(manager).await.unwrap()
}

pub async fn establish_sqlx_conn()->sqlx::Pool<sqlx::Postgres> {
    let sqlx_db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:wangc4@localhost".to_string());

    // set up connection pool
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&sqlx_db_connection_str)
        .await
        .expect("can't connect to database")
}