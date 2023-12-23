use actix_web::{Scope, Responder, web::{Data, Json, self}, post, HttpResponse, get};
use anyhow::Error;
use itertools::Itertools;
use serde::{Serialize, Deserialize};

use crate::{db::{DBPool, news::{news::{get_news, add_news_db, get_all_themes_db}, models::NewsInsert}, user::user::get_sources}, error, api::models::NewsBatchInfo};

use super::models::SearchOptions;


pub fn news_scope() -> Scope {
    Scope::new("/news")
        .service(news)
        .service(add_news)
        .service(get_all_sources)
        .default_service(web::route().to(HttpResponse::MethodNotAllowed))
}

#[post("/batch")]
pub async fn news(pool: Data<DBPool>,news_batch: Json<NewsBatchInfo>) -> actix_web::Result<impl Responder> {
    let res = web::block(move || {
        let mut conn = pool.get()?;
        Ok(get_news(news_batch.start_date, news_batch.amount, &news_batch.prefs, &mut conn))
    }).await?
    .map_err(|_: Error| error::ApiError::InternalError)?;
    // log::debug!("{res:?}");
    Ok(HttpResponse::Ok().json(res))
}

#[post("/add")]
pub async fn add_news(pool: Data<DBPool>, news_vec: Json<Vec<NewsInsert>>) -> actix_web::Result<impl Responder> {
    let res = web::block(move || {
        let mut conn = pool.get()?;
        add_news_db(news_vec.0,&mut conn)
    }).await?
    .map_err(|_| error::ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/search_info")]
pub async fn get_all_sources(pool: Data<DBPool>) -> actix_web::Result<impl Responder> {
    let res = web::block(move || {
        let mut conn = pool.get()?;
        let sources = get_sources(0, i64::MAX, &mut conn)?.into_iter().map(|x| x.name).collect_vec();
        let themes = get_all_themes_db(&mut conn)?.into_iter().map(|x| x.theme_name).collect_vec();
        Ok(SearchOptions { sources, themes })
    }).await?
    .map_err(|_: Error| error::ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(res))
}

