use crate::server::routes::{create_router, AppState};
use crate::server::watcher::watch;
use crate::Forge;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, RwLock};
use tracing::{error, info};

pub struct Server {}

impl Server {
    pub async fn run<T: ToString + Send + Sync + 'static>(watch_path: PathBuf, forge: Forge<T>) -> Result<(), ServerError> {
        let (tx, _rx) = broadcast::channel(100);
        let state = AppState {
            tx: tx.clone(),
            forge: Arc::new(RwLock::new(forge)),
        };

        // Initialize file watcher
        let _watcher = match watch(watch_path, tx) {
            Ok(watcher) => {
                info!("File watcher initialized successfully");
                watcher
            }
            Err(e) => {
                error!("Failed to initialize file watcher: {}", e);
                return Err(ServerError::Watcher(e));
            }
        };

        let app = create_router(state);
        let listener = TcpListener::bind("0.0.0.0:8866")
            .await
            .map_err(|e| ServerError::Io(e))?;

        info!("Server running on http://localhost:8866");

        let _ = axum::serve(listener, app).await;

        Ok(())
    }
}

#[derive(Debug)]
pub enum ServerError {
    Io(std::io::Error),
    Watcher(Box<dyn std::error::Error + Send + Sync>),
}