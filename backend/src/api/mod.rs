use std::sync::Arc;

use axum::{extract::{Path, State}, http::{header, response, Response, StatusCode}, response::IntoResponse, routing::get, Router};
use crate::{error::ApiError, setup::AppState};

use self::{admin::admin_router, auth::auth_router, feeds::feed_router, news::news_router};
pub mod auth;
pub mod news;
pub mod admin;
pub mod models;
pub mod feeds;
// /api
pub fn api_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .nest("/feed", feed_router())
        .nest("/auth", auth_router(state.clone()))
        .nest("/news",news_router())
        .nest("/admin",admin_router(state))
        .route("/cors_proxy/*link", get(cors_proxy) )
}

pub async fn cors_proxy(Path(link): Path<String>) -> Result<impl IntoResponse,ApiError> {
    tracing::info!("Proxying: {link}");
    let response = reqwest::get(link).await.map_err(|_| ApiError::InternalError)?.text().await.map_err(|_| ApiError::InternalError)?;
    Ok(response)
}
