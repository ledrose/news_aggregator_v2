use actix_web::{post, web::{Data, Json, self}, Responder, HttpResponse};
use crate::{db::{models::*, DBPool, user::{add_user_inter, auth_inter}}, error::ApiError};

#[post("/add")]
pub async fn add_user(pool: Data<DBPool>,user: Json<UserRegister>) -> actix_web::Result<impl Responder> {
    let user_db = web::block(move || {
        let mut conn = pool.get()?;
        add_user_inter(&user, &mut conn)
    })
    .await?
    .map_err(|_| ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(user_db))
}


#[post("/auth")]
pub async fn auth(pool: Data<DBPool>,user: Json<UserForm>) -> actix_web::Result<impl Responder> {
    let user_db = web::block(move || {
        let mut conn = pool.get()?;
        auth_inter(&user, &mut conn)
    }).await?
    .map_err(|_| ApiError::LoginError)?;
    Ok(HttpResponse::Ok().json(user_db))
}