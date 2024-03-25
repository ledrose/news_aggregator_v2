use std::{fmt::format, sync::Arc};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{body::Body, extract::{Request, State}, http::{header, HeaderMap, Response, StatusCode}, middleware::{self, Next}, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde_json::json;
use rand_core::OsRng;

use crate::{api::models::TokenClaims, auth_middleware, db::user::{models::{Role, User, UserForm, UserRegister, UserWithRole}, user::{add_user_inter, get_user_db}}, error::ApiError, jwt_middleware, setup::AppState};


// /auth
pub fn auth_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login",post(login_user))
        .route("/logout", get(logout))
		.route("/getme", get(get_me)
			.route_layer(auth_middleware!(state)))
}


pub async fn login_user(State(state): State<Arc<AppState>>,Json(data):Json<UserForm>) -> Result<impl IntoResponse, ApiError> {
	let conn = &state.db.get().await.unwrap();
	let user_info = conn.interact(move |conn| get_user_db(data.email, conn)).await??
		.ok_or(ApiError::LoginError)?;
	let hash = PasswordHash::new(&user_info.0.passwd_hash).map_err(|_| ApiError::LoginError)?;
	Argon2::default().verify_password(data.password.as_bytes(), &hash).map_err(|_| ApiError::LoginError)?;
	let now = chrono::Utc::now();
	let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::try_minutes(60).unwrap()).timestamp() as usize;
	let claims = TokenClaims {
		sub: user_info.0.email.clone(),
		iat,
		exp,
	};
	let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(state.env.jwt_secret.as_ref())).unwrap();
	let cookie = Cookie::build(("token", token.to_owned()))
		.path("/")
		.max_age(time::Duration::hours(1))
		.same_site(SameSite::Lax)
		.http_only(true);
	let mut response = Response::new(json!({"status":"success","token":token,"email":user_info.0.email,"role":user_info.1.name}).to_string());
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
	let user = conn.interact(move |conn| add_user_inter(data, hashed_password, conn)).await??;
	Ok(Json(user))
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

pub async fn get_me(State(state): State<Arc<AppState>>, Extension(user_info):Extension<(User,Role)>) -> impl IntoResponse {
	let user = UserWithRole {
		id: user_info.0.id,
		email: user_info.0.email,
		role: user_info.1.name,
	};
	Json(user)
}

// #[post("/role")]
// pub async fn get_user_role(pool: Data<DBPool>,session: Session) -> actix_web::Result<impl Responder>  {
//     if let Some(email) = session.get::<String>("email")? {
//         let role = web::block(move || {
//             let mut conn = pool.get()?;
//             get_role_db(email.as_str(), &mut conn)
//         }).await?
//             .map_err(|_| ApiError::InternalError)?;
//         Ok(HttpResponse::Ok().json(role))
//     } else {
//         Err(ApiError::NotLoggedError.into())
//     }
// }