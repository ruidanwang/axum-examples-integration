use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

use bb8_redis::bb8;

type ConnectionPool = Pool<RedisConnectionManager>;

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
    let key = parms.get("key").unwrap();
    let value = parms.get("value").unwrap();
    let mut conn = pool.get().await.map_err(internal_error)?;
    let result: String = conn.set(key, value).await.map_err(internal_error)?;
    Ok(result)
}

pub async fn lower_cmd_get(
    Path(key): Path<String>,
    State(pool): State<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let result: String = redis::cmd("get").arg(&key).query_async(&mut *conn).await.map_err(internal_error)?;
    Ok(result)
}

// pub async fn redis_transaction(
//     Path(key): Path<String>,
//     State(pool): State<ConnectionPool>,)
// {
//     let mut conn = pool.get().await.unwrap();
//     let (new_val,) : (isize,) = redis::transaction(&mut *conn, &[key], |conn, pipe| {
//         let old_val : isize = conn.get(key).;
//         pipe
//             .set(key, old_val + 1).ignore()
//             .get(key).query(conn)
//     })?;

    
// }

// use redis::Commands;
// let key = "the_key";
// let (new_val,) : (isize,) = redis::transaction(&mut con, &[key], |con, pipe| {
//     let old_val : isize = con.get(key)?;
//     pipe
//         .set(key, old_val + 1).ignore()
//         .get(key).query(con)
// })?;

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
