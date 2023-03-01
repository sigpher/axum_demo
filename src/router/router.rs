use crate::handler::*;
use axum::routing::{get, post};
use axum::Router;



pub fn get_routers() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .fallback(handler_404)
}
