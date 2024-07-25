pub mod sse;
pub mod ws;
pub mod websockets;

pub mod types;

use axum::routing::{get, post};
use tokio::sync::broadcast;
use tower_http::services::{ServeDir, ServeFile};

use types::Channels;
// use core::net::SocketAddr;

pub fn app() -> axum::Router {
    let (tx, _rx) = broadcast::channel(100);
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    let notifications_router = axum::Router::new()
        .fallback_service(serve_dir)
        .route("/ws/:user_id", get(ws::ws_handler))
        .route("/sse/:user_id", get(sse::sse_handler))
        .route("/admin/send_notification/:user_id", post(ws::send_notification_handler))
        .with_state(Channels {tx})
        ;
    axum::Router::new().nest("/notify", notifications_router)
}