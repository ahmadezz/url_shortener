use std::env;

use axum::{
    routing::{get, post},
    Router,
};
use handler::{get_short_url, redirect_url};
use migration::{Migrator, MigratorTrait};
use model::AppState;
use sea_orm::Database;
use std::net::SocketAddr;

mod handler;
mod model;
mod data;

/// Generate signal for graceful shutdown of the app
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

/// Entrypoint of the service
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get connection to specified postgres database
    let db_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in environment variables");
    let db_conn = Database::connect(db_url).await?;

    // apply all pending migrations on app start
    if Migrator::up(&db_conn, None).await.is_err() {
        panic!("Failed to apply migrations")
    };

    // get base url for the app
    let base_url = env::var("BASE_URL").expect("BASE_URL is not set in environment variables");

    // Appstate which stores the db connection and base url to be shared in handlers
    let state = AppState {
        base_url,
        db: db_conn,
    };
    // create routes
    let app = Router::new()
        .route("/getShortUrl", post(get_short_url))
        .route("/:id", get(redirect_url))
        .with_state(state);
    // create an internet socket address, either IPv4 or IPv6.
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}
