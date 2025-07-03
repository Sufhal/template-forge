use tokio::net::TcpListener;

use crate::server::routes::create_router;

pub struct Server {

}

impl Server {
    pub async fn run() -> Result<(), ServerError> {
        let app = create_router();
        let listener = TcpListener::bind("0.0.0.0:8866")
            .await
            .map_err(|e| ServerError::Io(e))?;

        axum::serve(listener, app).await;
        Ok(())
    }
}

pub enum ServerError {
    Io(std::io::Error),
}