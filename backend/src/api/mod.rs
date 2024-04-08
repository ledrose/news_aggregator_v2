use std::sync::Arc;

use axum::{extract::{Path, State}, http::{header, response, Response, StatusCode}, response::IntoResponse, routing::get, Router};
use crate::{error::ApiError, setup::AppState};

use self::{admin::admin_router, auth::auth_router, news::news_router};
pub mod auth;
pub mod news;
pub mod admin;
pub mod models;
// /api
pub fn api_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth_router(state.clone()))
        .nest("/news",news_router())
        .nest("/admin",admin_router(state))
        .route("/cors_proxy/*link", get(cors_proxy) )
}

pub async fn cors_proxy(Path(link): Path<String>) -> Result<impl IntoResponse,ApiError> {
    tracing::info!("Proxying: {link}");
    let response = reqwest::get(link).await.map_err(|_| ApiError::InternalError)?.text().await.map_err(|_| ApiError::InternalError)?;
    // headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    // headers.insert("Access-Control-Allow-Methods", "GET, PUT, PATCH, POST, DELETE".parse().unwrap());
    // headers.insert("Access-Control-Allow-Headers", "Accept".parse().unwrap());
    // let req = [
    //     (header::ACCESS_CONTROL_ALLOW_ORIGIN,"*"),
    //     (header::ACCESS_CONTROL_ALLOW_METHODS,"GET, PUT, PATCH, POST, DELETE"),
    //     (header::ACCESS_CONTROL_ALLOW_HEADERS,"Accept"),
    //     (header::CONTENT_TYPE,"")
    // ];
    // println!("{response}");
    Ok(response)
}
