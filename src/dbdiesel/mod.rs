pub mod schema;
pub mod model;
pub mod diesel_;

use axum::routing::{get,post};
use diesel::pg::PgConnection;



pub fn app(diesel_pool: deadpool_diesel::Pool<deadpool_diesel::Manager<PgConnection>>) -> axum::Router {
    let diesel_router = axum::Router::new()
        .route("/get_all", get(diesel_::list_users))
        .route("/create", post(diesel_::create_user))
        // .route("/update", get(diesel_::update_user))
        .route("/delete_users/:ids", get(diesel_::delete_users))
        .with_state(diesel_pool)
    ;
    axum::Router::new().nest("/diesel", diesel_router)
}
