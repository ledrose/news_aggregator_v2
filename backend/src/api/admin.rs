use actix_session::SessionExt;
use actix_web::{Scope, guard, web::{self, Data}, post, Responder, HttpResponse, get};
use itertools::Itertools;
use serde_json::json;

use crate::{db::{user::user::get_all_users, DBPool}, error::ApiError};

use super::models::UserAnswer;


pub fn admin_scope() -> Scope {
    Scope::new("/admin")
        .guard(guard::fn_guard(|ctx| {
            if let Ok(Some(role)) = ctx.get_session().get::<String>("role") {
                if role == "admin" {
                    return true;
                }
            }
            false
        }))
        .service(get_user_list)
}

#[get("/users")]
pub async fn get_user_list(pool: Data<DBPool>) -> actix_web::Result<impl Responder> {
    let user_list = web::block(move || {
        let mut conn = pool.get()?;
        get_all_users(&mut conn)
    }).await?
        .map_err(|_| ApiError::LoginError)?;
    let user_answer: Vec<UserAnswer> = user_list.into_iter().map(|x| x.into()).collect();
    Ok(HttpResponse::Ok().json(user_answer))
}