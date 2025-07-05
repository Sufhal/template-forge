use crate::common::utils::create_forge;
use std::path::PathBuf;
use template_forge::Server;

mod common;
mod components;
mod templates;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let forge = create_forge(true);
    Server::run(PathBuf::from("./examples"), forge)
        .await
        .unwrap();
}
