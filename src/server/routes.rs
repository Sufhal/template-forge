use axum::{response::Html, routing::get, Router};

pub fn create_router() -> Router<_> {
    Router::new()
        .route("/", get(index_handler))
}

fn index_handler() -> Html<&'static str> {
    Html(include_str!("index.html"))
}