use std::sync::Arc;

use axum::{body::Body, extract::{Request, State}, middleware::Next, http::{StatusCode, header}, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;

use crate::{api::models::TokenClaims, db::user::{models::{Role, UserForm, UserRegister}, user::get_user_db}, error::ApiError, setup::AppState};


pub async fn auth(cookie_jar: CookieJar,State(state): State<Arc<AppState>>,mut request: Request<Body>, next: Next) -> Result<impl IntoResponse, ApiError> {
	let conn = &state.db.get().await.unwrap();
	let token = cookie_jar
		.get("token")
		.map(|cookie| cookie.value().to_string())
		.or_else(|| {
			request.headers()
				.get(header::AUTHORIZATION)
				.and_then(|auth_h| auth_h.to_str().ok())
				.and_then(|auth_h| {
					auth_h.strip_prefix("Bearer ").map(|token| token.to_owned())
				})
		}).ok_or(ApiError::Unauthorized)?;
	let email = decode::<TokenClaims>(
		&token,
		&DecodingKey::from_secret(state.env.jwt_secret.as_ref()),
		&Validation::default()
	).map_err(|_| ApiError::Unauthorized)?.claims.sub;
	let user_info = conn.interact(move |conn| get_user_db(email, conn)).await??;
	let user_info = user_info.ok_or(ApiError::Unauthorized)?;
	if user_info.1.name == "admin" {
		return  Err(ApiError::Unauthorized);
	}
	Ok(next.run(request).await)
}


pub async fn login_user(State(state): State<Arc<AppState>>,Json(data):Json<UserForm>) -> Result<impl IntoResponse, ApiError> {
	
	Ok(Json(json!({})))
}

pub async fn register_user(State(state): State<Arc<AppState>>,Json(data):Json<UserRegister>) -> Result<impl IntoResponse, ApiError> {
	
	Ok(Json(json!({})))
}

pub async fn logout() -> Result<impl IntoResponse,ApiError> {

	Ok(StatusCode::OK)
}