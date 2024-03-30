use core::fmt;
use std::{future::Future, sync::Arc};
use axum::{body::Body, extract::{Request, State}, http::{header, Response, StatusCode}, middleware::{self, Next}, response::IntoResponse, Json, ServiceExt};
use axum_extra::extract::{CookieJar};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::{api::models::TokenClaims, db::user::{ user::{add_user_inter, get_user_db}}, error::ApiError, setup::AppState};

pub async fn auth(cookie_jar: CookieJar,State(state): State<Arc<AppState>>,mut request: Request<Body>, next: Next,role: Option<&'static str>) -> Result<Response<Body>, ApiError> {
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
	if role.is_some_and(|role| user_info.1.name != role) {
		return  Err(ApiError::Unauthorized);
	}
	request.extensions_mut().insert(user_info);
	Ok(next.run(request).await)
}

#[macro_export]
macro_rules! auth_middleware {
	($state:expr) => {
        {
			axum::middleware::from_fn_with_state(
				$state.clone(), 
				move |cookie_jar, state,request, next| {$crate::jwt_middleware::auth(cookie_jar,state,request,next,None) }
			)
		}
    };
    ($state:expr,$role:expr) => {
        {
			axum::middleware::from_fn_with_state(
				$state.clone(), 
				move |cookie_jar, state,request, next| {$crate::jwt_middleware::auth(cookie_jar,state,request,next,Some($role)) }
			)
		}
    };
}

// middleware::from_fn_with_state(
// 	state.clone(), 
// 	move |cookie_jar, state,request, next| {jwt_middleware::auth(cookie_jar,state,request,next,Some("admin")) }
// )