use actix_session::Session;
use actix_web::{post, web::{Data, Json, self}, Responder, HttpResponse, Scope, get};
use serde_json::json;
use crate::{db::{DBPool, user::{models::*, user::*}}, error::ApiError};

pub fn auth_scope() -> Scope {
    Scope::new("/auth")
        .service(register)
        .service(login)
        .service(logout)
        // .service(get_user_role)
}



#[post("/register")]
pub async fn register(pool: Data<DBPool>,user: Json<UserRegister>) -> actix_web::Result<impl Responder> {

    let user_db = web::block(move || {
        let mut conn = pool.get().map_err(|_| ApiError::InternalError)?;
        add_user_inter(&user, &mut conn)
    })
    .await??;
    Ok(HttpResponse::Ok().json(user_db))
}


#[post("/login")]
pub async fn login(pool: Data<DBPool>,user: Json<UserForm>,session: Session) -> actix_web::Result<impl Responder> {
    let user_db = web::block(move || {
        let mut conn = pool.get()?;
        auth_inter(&user, &mut conn)
    }).await?
        .map_err(|_| ApiError::LoginError)?;
    session.renew();
    session.insert("email", user_db.email.as_str())?;
    session.insert("role", user_db.role.as_str())?;
    Ok(HttpResponse::Ok().json(user_db))
}

#[get("/logout")]
pub async fn logout(session: Session) -> actix_web::Result<impl Responder> {
    session.renew();
    Ok(HttpResponse::Ok().json(json!({"answer":"ok"})))
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