use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use tokio::sync::broadcast::Sender;
use tracing::{error, info, trace, warn};

pub fn watch(
    path: PathBuf,
    tx: Sender<String>,
) -> Result<RecommendedWatcher, Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting file watcher for path: {}", path.display());

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                // Filter for file modification events
                if matches!(event.kind, EventKind::Modify(_)) {
                    if let Some(path) = event.paths.first() {
                        if path.is_file() {
                            if let Some(file_name) = path.file_name() {
                                info!("File modification detected");
                                let msg = format!("File modified: {}", file_name.to_string_lossy());
                                if let Err(e) = tx.send(msg) {
                                    warn!("Failed to send file event: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        },
        Config::default(),
    )?;

    watcher.watch(&path, RecursiveMode::Recursive)?;
    info!(
        "File watcher successfully initialized for: {}",
        path.display()
    );

    Ok(watcher)
}
