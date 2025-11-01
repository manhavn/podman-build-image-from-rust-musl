use crate::handlers;
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route(
            "/users",
            get(handlers::get_users).post(handlers::create_user),
        )
        .route(
            "/users/{id}",
            get(handlers::get_user_by_id)
                .put(handlers::update_user)
                .delete(handlers::delete_user),
        )
}
