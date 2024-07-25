use axum::{extract::{ Path, State, }, response::{sse::Event, Sse}};
use futures_util::Stream;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use std::{convert::Infallible, time::Duration};
use crate::notifications::types::{Channels, MessageType, Notification};


/// Handle Server Sent Event connection
pub async  fn sse_handler(
    State(state): State<Channels>,
    Path(user_id): Path<String>
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {

    // Get broadcast receiver
    let broadcast_rx = state.tx.subscribe();
    // clone user id to use in closures below
    let user_id_filter = user_id.clone();

    // convert broadcast receiver into stream
    let stream = BroadcastStream::new(broadcast_rx)
    // filter notifications for current user
    .filter(move |f| {
        let value = match f {
            Ok(notification) => notification,
            Err(e) => &Notification { user_id: user_id_filter.clone(), message: MessageType::Error(e.to_string()) },
        };
        return user_id_filter == value.user_id;
    })
    // Format notifications into Server Sent Events
    .map(move |f| { 
        let user_id = user_id.to_string();
        let value = match f {
            Ok(notification) => notification,
            Err(e) => Notification { user_id, message: MessageType::Error(e.to_string()) },
        };
        let data = serde_json::to_string(&value).unwrap_or_default();
        Ok(Event::default().data(data))
    });
    
    // Send response to client
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}