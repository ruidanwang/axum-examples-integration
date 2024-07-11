pub mod company;
pub mod dept;
// pub mod handerr;


use axum::routing::{get, post};


pub fn app() -> axum::Router {
    let dept_router = axum::Router::new()
        .route("/get", get(dept::handler_get_dept))
        .route("/get_all_header", get(dept::get_all_header))
        .route("/get_body_string", post(dept::get_body_string))
        .route("/get_mutil_extra", get(dept::get_mutil_extra));

    let company_router = axum::Router::new()
        .route("/get", get(company::get_new_headers));

    // let err_router =axum::Router::new()
    //     // .route("/route_fall_svc", get(handerr::router_fallible_service))
    //     .route("/route_fall_middle", get(|| async {}))
    //     .layer(ServiceBuilder::new()
    //         // `timeout` will produce an error if the handler takes
    //         // too long so we must handle those
    //         .layer(HandleErrorLayer::new(handerr::handle_timeout_error))
    //         .timeout(Duration::from_secs(30))
    // );

    axum::Router::new()
        .nest("/dept", dept_router)
        .fallback(dept::fallback)
        .nest("/company", company_router)
        // .nest("/error", err_router)
}
