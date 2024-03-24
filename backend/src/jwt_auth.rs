use std::sync::Arc;

use axum::{body::Body, extract::{Request, State}, middleware::Next, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde_json::json;
use tracing::{event, Level};

use crate::{db::user::models::UserRegister, error::ApiError, setup::AppState};



pub async fn auth(cookie_jar: CookieJar,State(state): State<Arc<AppState>>,mut request: Request<Body>, next: Next) -> Result<impl IntoResponse, ApiError> {
	event!(Level::DEBUG,"using auth middleware");
	Ok(next.run(request).await)
}

pub async fn register_user(State(state): State<Arc<AppState>>,Json(data):Json<UserRegister>) -> Result<impl IntoResponse, ApiError> {
	
	Ok(Json(json!({})))
}

pub async fn logout() -> Result<impl IntoResponse,ApiError> {

	Ok(StatusCode::OK)
}