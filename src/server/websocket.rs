use crate::server::routes::AppState;
use crate::Forge;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tracing::{info, warn};

pub async fn websocket_connection<T: ToString + Send + Sync + 'static>(socket: WebSocket, state: Arc<AppState<T>>) {
    let (mut sender, _receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    info!("websocket connected");

    // Task to handle outgoing messages to the client
    let send_task = tokio::spawn(async move {
        while let Ok(_) = rx.recv().await {
            if sender.send(Message::Text(Utf8Bytes::from(msg))).await.is_err() {
                warn!("Failed to send message to client");
                break;
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
    }
}

fn rebuild_templates<T: ToString + Send + Sync + 'static>(forge: &Forge<T>) {
    for name in forge.get_templates() {}
}