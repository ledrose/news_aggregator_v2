use actix_web::{Scope, Responder, web::{Data, Json, self}, post, HttpResponse};
use anyhow::Error;
use serde::{Serialize, Deserialize};

use crate::{db::{DBPool, news::{news::{get_news, add_news_db}, models::NewsInsert}}, error, api::models::NewsBatchInfo};


pub fn news_scope() -> Scope {
    Scope::new("/news")
        .service(news)
        .service(add_news)
        .default_service(web::route().to(HttpResponse::MethodNotAllowed))
}

#[post("/batch")]
pub async fn news(pool: Data<DBPool>,news_batch: Json<NewsBatchInfo>) -> actix_web::Result<impl Responder> {
    let res = web::block(move || {
        let mut conn = pool.get()?;
        Ok(get_news(news_batch.max_id, news_batch.amount, &mut conn))
    }).await?
    .map_err(|_: Error| error::ApiError::InternalError)?;
    log::debug!("{res:?}");
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