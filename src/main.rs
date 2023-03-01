mod database;
mod handler;
mod model;
mod router;

use router::get_routers;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = get_routers();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
