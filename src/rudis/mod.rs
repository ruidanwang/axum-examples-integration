
pub mod rudis;

use axum::routing::get;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;



pub fn app(rudis_pool: Pool<RedisConnectionManager>) -> axum::Router {
    let rudis_router = axum::Router::new()
        .route("/get/:id", get(rudis::get_value))
        .route("/set", get(rudis::set_key_value))
        // .route("/delete_users/:ids", get(rudis::delete_users))
        .with_state(rudis_pool)
    ;
    axum::Router::new().nest("/rudis", rudis_router)
}