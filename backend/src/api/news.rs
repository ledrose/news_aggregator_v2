use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use itertools::Itertools;

use crate::{api::models::NewsBatchInfo, db::{news::{models::NewsInsert, news::{add_news_db, get_all_themes_db, get_news}}, user::user::get_sources}, error::{self, ApiError}, setup::AppState};

use super::models::SearchOptions;


pub fn news_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/batch", post(news))
        .route("/add", post(add_news))
        .route("/search_info",get(get_all_sources))
}
pub async fn news(State(state): State<Arc<AppState>>,Json(mut news_batch): Json<NewsBatchInfo>) -> Result<impl IntoResponse,ApiError> {
    let conn = &state.db.get().await.unwrap();
    let res = conn.interact(move |conn| {
        get_news(news_batch.start_date, news_batch.amount, &mut news_batch.prefs, conn)
    }).await?;
    Ok(Json(res))
}

pub async fn add_news(State(state): State<Arc<AppState>>, Json(news_vec): Json<Vec<NewsInsert>>) -> Result<impl IntoResponse,ApiError> {
    let conn = &state.db.get().await.unwrap();
    let res = conn.interact(move |conn| {
        add_news_db(news_vec,conn)
    }).await??;
    Ok(Json(res))
}

pub async fn get_all_sources(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse,ApiError> {
    let conn = &state.db.get().await.unwrap();
    let res = conn.interact( |conn| {
        let sources = get_sources(0, i64::MAX, conn)?.into_iter().map(|x| x.name).collect_vec();
        let themes = get_all_themes_db(conn)?.into_iter().map(|x| x.theme_name).collect_vec();
        Ok::<SearchOptions,anyhow::Error>(SearchOptions { sources, themes })
    }).await??;
    Ok(Json(res))
}

