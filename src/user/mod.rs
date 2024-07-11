pub mod role;
pub mod user;

use axum::routing::{get, post};


pub fn app() -> axum::Router {
    let user_routers = axum::Router::new()
        .route("/get", post(user::handle_list)) 
        .route("/get_path_parm/:user_id", get(user::get_path_parm))
        .route("/get_query_parm", get(user::get_query_parm))
        .route("/post_json", post(user::json))
        .route("/get_user/:user_id", get(user::get_user))
        .route("/get_path_parm_mtil/:id/:name", get(user::get_path_parm_mtil))
        .route("/validate_post_json", post(user::validatejson))
        ;
        
    let role_routers = axum::Router::new()
        .route("/get", get(role::handle_get))
        .route("/get_wdq", get(role::handle_get_wdq));
    axum::Router::new()
        .nest("/role", role_routers)
        .nest("/user", user_routers)
}
