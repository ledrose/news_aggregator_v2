use std::sync::Arc;

use axum::Router;
use crate::setup::AppState;

use self::{admin::admin_router, news::news_router};
// pub mod auth;
pub mod news;
pub mod admin;
pub mod models;
// /api
pub fn api_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        // .service(auth::auth_scope())
        .nest("/news",news_router())
        .nest("/admin",admin_router(state))
}

