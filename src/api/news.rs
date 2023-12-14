use actix_web::{Scope, Responder, web::{Data, Json, self}, post, HttpResponse};
use anyhow::Error;
use serde::{Serialize, Deserialize};

use crate::{db::{DBPool, news::news::get_news}, error};

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
pub async fn news(pool: Data<DBPool>,user: Json<NewsBatchInfo>) -> actix_web::Result<impl Responder> {
    let res = web::block(move || {
        let mut conn = pool.get()?;
        Ok(get_news(user.id0, user.amount, &mut conn))
    }).await?
    .map_err(|_: Error| error::ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(res))
}
