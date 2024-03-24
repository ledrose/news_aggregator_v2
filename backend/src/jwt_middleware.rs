use std::{fmt::format, sync::Arc};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{body::Body, extract::{Request, State}, http::{header, Response, StatusCode}, middleware::Next, response::IntoResponse, Json};
use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde_json::json;
use rand_core::OsRng;

use crate::{api::models::TokenClaims, db::user::{models::{Role, UserForm, UserRegister}, user::{add_user_inter, get_user_db}}, error::ApiError, setup::AppState};


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
	let conn = &state.db.get().await.unwrap();
	let user_info = conn.interact(move |conn| get_user_db(data.email, conn)).await??
		.ok_or(ApiError::LoginError)?;
	let hash = PasswordHash::new(&user_info.0.passwd_hash).map_err(|_| ApiError::LoginError)?;
	Argon2::default().verify_password(&data.password.as_bytes(), &hash).map_err(|_| ApiError::LoginError)?;
	let now = chrono::Utc::now();
	let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::try_minutes(60).unwrap()).timestamp() as usize;
	let claims = TokenClaims {
		sub: user_info.0.email,
		iat,
		exp,
	};
	let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(state.env.jwt_secret.as_ref())).unwrap();
	let cookie = Cookie::build(("token", token.to_owned()))
		.path("/")
		.max_age(time::Duration::hours(1))
		.same_site(SameSite::Lax)
		.http_only(true);
	let mut response = Response::new(json!({"status":"success","token":token}).to_string());
	response.headers_mut().insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
	Ok(response)
}

pub async fn register_user(State(state): State<Arc<AppState>>,Json(data):Json<UserRegister>) -> Result<impl IntoResponse, ApiError> {
	let conn = &state.db.get().await.unwrap();
	let email = data.email.clone();
	let user_info = conn.interact(move |conn| get_user_db(email, conn)).await??;
	if user_info.is_some() {
		return Err(ApiError::UserAlreadyExists);
	}
	let salt = SaltString::generate(&mut OsRng);
	let hashed_password = Argon2::default()
		.hash_password(data.password.as_bytes(), &salt)
		.map_err(|_| ApiError::InternalError)?
		.to_string();
	let conn = &state.db.get().await.unwrap();
	let user = conn.interact(move |conn| add_user_inter(data, hashed_password, conn)).await??;
	Ok(Json(json!({"user":user})))
}

pub async fn logout() -> Result<impl IntoResponse,ApiError> {
	let cookie = Cookie::build(("token", ""))
		.path("/")
		.max_age(time::Duration::hours(-1))
		.same_site(SameSite::Lax)
		.http_only(true);

	let mut response = Response::new(json!({"status": "success"}).to_string());
	response
		.headers_mut()
		.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
	Ok(response)
}