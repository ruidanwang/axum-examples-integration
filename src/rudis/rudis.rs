use std::collections::HashMap;

use anyhow::Ok;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path, Query, State},
    http::{request::Parts, StatusCode},
};
use bb8::{Pool, PooledConnection};
use bb8_redis::{RedisConnectionManager,redis::cmd,};
use redis::AsyncCommands;

use bb8_redis::bb8;
use serde::{Deserialize,Serialize};

type ConnectionPool = Pool<RedisConnectionManager>;

#[derive(Deserialize,Serialize)]
pub struct Rudis{
    cmd:String,
    key:String,
    value:String
}

pub async fn get_value(
    Path(key): Path<String>,
    State(pool): State<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let result: String = conn.get(key).await.map_err(internal_error)?;
    Ok(result)
}

pub async fn set_key_value(
    Query(parms): Query<HashMap<String, String>>,
    State(pool): State<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let key = &parms.get("key").unwrap();
    let value = &parms.get("value").unwrap();
    let mut conn = pool.get().await.map_err(internal_error)?;
    let result: String = conn.set(key,value).await.map_err(internal_error)?;
    Ok(result)
}

pub async fn lower_cmd(
    Path(key): Path<String>,
    State(pool): State<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let result = cmd("get").arg(&key).query_async(&mut *conn).await.map_err(internal_error)?;
    Ok(result)
}


// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(PooledConnection<'static, RedisConnectionManager>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    ConnectionPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = ConnectionPool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub async fn using_connection_extractor(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let result: String = conn.get("foo").await.map_err(internal_error)?;

    Ok(result)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
