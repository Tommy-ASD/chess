use axum::{response::Html, routing::get, Router};
use clap::Parser;
use std::{collections::HashSet, net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, action, default_value = "8080")]
    port: u16,
}

async fn index() -> Html<&'static str> {
    Html(":3")
}

pub struct AppState {}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let app_state = Arc::new(AppState {});

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);
    tokio::spawn(
        hyper::Server::bind(&SocketAddr::from(([0, 0, 0, 0], args.port)))
            .serve(app.into_make_service()),
    );
    loop {}
}
