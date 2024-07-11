pub mod sqlx_;

use axum::{routing::get,extract::State};
use sqlx::{ Pool, Postgres};

pub fn app(State(pool): State<Pool<Postgres>>) -> axum::Router {
    let sqlx_router = axum::Router::new()
        .route("/get_one", get(sqlx_::get_one))
        .route("/get_many", get(sqlx_::get_many)).with_state(pool)
    ;
    axum::Router::new().nest("/sqlx", sqlx_router)
}
