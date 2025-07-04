use crate::server::websocket::websocket_connection;
use crate::Forge;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::{IntoResponse, Response};
use axum::{response::Html, routing::get, Json, Router};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tower_http::services::ServeDir;


pub struct AppState<T: ToString + Send + Sync + 'static> {
    pub tx: broadcast::Sender<String>,
    pub forge: Arc<RwLock<Forge<T>>>,
}

impl<'a, T: ToString + Send + Sync + 'static> Clone for AppState<T> {
    fn clone(&self) -> Self {
        AppState {
            tx: self.tx.clone(),
            forge: Arc::clone(&self.forge),
        }
    }
}

pub fn create_router<'a, T: ToString + Send + Sync + 'static>(state: AppState<T>) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .nest_service("/assets", assets_handler())
        .route("/ws", get(websocket_handler))
        .with_state(Arc::new(state))
}

async fn index_handler() -> Html<&'static str> {
    Html(include_str!("./../static/index.html"))
}

fn assets_handler() -> ServeDir {
    ServeDir::new("./src/static/assets")
}

pub async fn websocket_handler<'a, T>(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState<T>>>,
) -> Response
where
    T: ToString + Send + Sync + 'static,
{
    let state_clone = Arc::clone(&state);

    ws.on_upgrade(move |socket| async move {
        websocket_connection(socket, state_clone).await;
    })
}

async fn get_templates_handler<'a, T: ToString + Send + Sync + 'static>(State(state): State<Arc<AppState<T>>>) -> impl IntoResponse {
    let forge = state.forge.read().await;
    let templates = forge.get_templates();
    Json(templates)
}