use axum::routing::get;

pub mod valid;
pub mod reqwest_req;


pub fn app() -> axum::Router {
    let reqwest_router = axum::Router::new()
        .route("/typed", get(reqwest_req::typed_json))
        .route("/format", get(reqwest_req::typed_json_resp));
    axum::Router::new().nest("/reqwest", reqwest_router)
}

