use crate::errors::ForgeError;
use crate::server::routes::AppState;
use crate::Forge;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info, warn};

pub async fn websocket_connection<T, S>(socket: WebSocket, state: Arc<AppState<T, S>>)
where
    T: ToString + Send + Sync + 'static,
    S: Serialize + Send + Sync + 'static,
{
    let (mut sender, _receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    info!("websocket connected");
    state.tx.send("Initial connection".to_string()).unwrap();

    // Task to handle outgoing messages to the client
    let send_task = tokio::spawn(async move {
        let state_clone = Arc::clone(&state);
        let forge_clone = state_clone.forge.clone();
        while let Ok(_) = rx.recv().await {
            let forge = forge_clone.read().await;
            match rebuild_templates(&*forge) {
                Ok(rebuilt_templates) => {
                    let json = serde_json::to_string(&rebuilt_templates).unwrap();
                    if sender
                        .send(Message::Text(Utf8Bytes::from(json)))
                        .await
                        .is_err()
                    {
                        warn!("Failed to send message to client");
                        break;
                    }
                }
                Err(err) => {
                    error!("Failed to rebuild templates: {:#?}", err);
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
    }
}

fn rebuild_templates<T, S>(forge: &Forge<T, S>) -> Result<HashMap<String, String>, ForgeError>
where
    T: ToString + Send + Sync + 'static,
    S: Serialize + Send + Sync + 'static,
{
    let mut templates = HashMap::new();
    for name in forge.get_templates() {
        let render = forge.render_with_default(&name)?;
        templates.insert(name, render);
    }
    Ok(templates)
}
