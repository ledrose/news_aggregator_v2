use actix_web::{post, web::{Data, Json, self}, Responder, HttpResponse, Scope};
use crate::{db::{DBPool, user::{models::*, user::*}}, error::ApiError};

pub fn auth_scope() -> Scope {
    Scope::new("/auth")
        .service(register)
        .service(login)
}

#[post("/register")]
pub async fn register(pool: Data<DBPool>,user: Json<UserRegister>) -> actix_web::Result<impl Responder> {
    let user_db = web::block(move || {
        let mut conn = pool.get()?;
        add_user_inter(&user, &mut conn)
    })
    .await?
    .map_err(|_| ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(user_db))
}


#[post("/login")]
pub async fn login(pool: Data<DBPool>,user: Json<UserForm>) -> actix_web::Result<impl Responder> {
    let user_db = web::block(move || {
        let mut conn = pool.get()?;
        auth_inter(&user, &mut conn)
    }).await?
    .map_err(|_| ApiError::LoginError)?;
    Ok(HttpResponse::Ok().json(user_db))
}