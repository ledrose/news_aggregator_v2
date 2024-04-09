use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::{get, post}, Extension, Json, Router};

use crate::{db::user::models::{Role, User}, db, error::ApiError, setup::AppState};

use super::models::FeedInfo;

// /auth
pub fn feed_router() -> Router<Arc<AppState>> {
    Router::new()
		.route("/", get(get_user_feed))
		.route("/add", post(add_user_feed))
		.route("/delete", post(delete_user_feed))

}

pub async fn get_user_feed(State(state): State<Arc<AppState>>,Extension(user_info):Extension<(User,Role)>) -> Result<impl IntoResponse, ApiError> {
	let conn = &state.db.get().await.unwrap();
    let res = conn.interact(move |conn| {
		db::feeds::feeds::get_feeds(user_info.0.id,conn)
	}).await??;
    Ok(Json(res))
}

pub async fn add_user_feed(State(state): State<Arc<AppState>>,Extension(user_info):Extension<(User,Role)>,Json(feed_info): Json<FeedInfo>) -> Result<impl IntoResponse, ApiError> {
	let conn = &state.db.get().await.unwrap();
    let _ = conn.interact(move |conn| {
		db::feeds::feeds::add_feed(user_info.0.id,feed_info,conn)
	}).await??;
    Ok(())
}

pub async fn delete_user_feed(State(state): State<Arc<AppState>>,Extension(user_info):Extension<(User,Role)>,Json(feed_info): Json<FeedInfo>) -> Result<impl IntoResponse, ApiError> {
	let conn = &state.db.get().await.unwrap();
    let _ = conn.interact(move |conn| {
		db::feeds::feeds::delete_feed(user_info.0.id,&feed_info.name,conn)
	}).await??;
    Ok(())
}
