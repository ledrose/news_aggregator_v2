use actix_web::{Scope, Responder, web::{Data, Json, self}, post, HttpResponse};
use anyhow::Error;
use serde::{Serialize, Deserialize};

use crate::{db::{DBPool, news::{news::{get_news, add_news_db}, models::NewsInsert}}, error};

#[derive(Serialize,Deserialize,Debug)]
pub struct NewsBatchInfo {
    id0: i64,
    amount: i64
}

pub fn news_scope() -> Scope {
    Scope::new("/news")
        .service(news)
}

#[post("/batch")]
pub async fn news(pool: Data<DBPool>,news_batch: Json<NewsBatchInfo>) -> actix_web::Result<impl Responder> {
    let res = web::block(move || {
        let mut conn = pool.get()?;
        Ok(get_news(news_batch.id0, news_batch.amount, &mut conn))
    }).await?
    .map_err(|_: Error| error::ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn add_news(pool: Data<DBPool>, news_vec: Vec<NewsInsert>) -> actix_web::Result<impl Responder> {
    let res = web::block(move || {
        let mut conn = pool.get()?;
        add_news_db(news_vec,&mut conn)
    }).await?
    .map_err(|_| error::ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(res))
}