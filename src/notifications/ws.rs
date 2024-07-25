use axum::{extract::{ConnectInfo, Path, State, WebSocketUpgrade}, response::IntoResponse, Json};
// use futures_util::Stream;
// use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use core::net::SocketAddr;
// use std::{convert::Infallible, time::Duration};

use crate::notifications::{types::{Channels, MessageType, Notification, NotificationMessage}, websockets::handle_socket};

/// Handle Websocket connection
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Channels>,
    Path(user_id): Path<String>
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state, user_id))
}

/// Forward message in request body to broadcast channel
pub async fn send_notification_handler(
    State(state): State<Channels>,
    Path(user_id): Path<String>,
    Json(payload): Json<NotificationMessage>
) -> impl IntoResponse {
    let notification = Notification {
        user_id,
        message: MessageType::Data(payload.message)
    };
    let _ = state.tx.send(notification);
    Json(serde_json::json!({"status": 200}))
}