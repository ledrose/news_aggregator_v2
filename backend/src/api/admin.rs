use actix_session::SessionExt;
use actix_web::{Scope, guard, web::{self, Data, Json, Query}, post, Responder, HttpResponse, get};

use crate::{db::{user::user::{get_all_users, get_source_themes}, DBPool, news::news::get_sources_db}, error::ApiError, api::models::{PaginateData, SourceThemesResp}};

use super::models::UserAnswer;


pub fn admin_scope() -> Scope {
    Scope::new("/admin")
        .guard(guard::fn_guard(|ctx| {
            if let Ok(Some(role)) = ctx.get_session().get::<String>("role") {
                if role == "admin" {
                    println!("GuardCheck True");
                    return true;
                }
            }
            println!("GuardCheck False");
            false
        }))
        .service(get_user_list)
        .service(get_sources)
}

#[get("/users")]
pub async fn get_user_list(pool: Data<DBPool>) -> actix_web::Result<impl Responder> {
    let user_list = web::block(move || {
        let mut conn = pool.get()?;
        get_all_users(&mut conn)
    }).await?
        .map_err(|_| ApiError::InternalError)?;
    let user_answer: Vec<UserAnswer> = user_list.into_iter().map(|x| x.into()).collect();
    Ok(HttpResponse::Ok().json(user_answer))
}

// #[post("/sourcethemes/")]
// pub async fn get_sourcethemes(pool: Data<DBPool>,input: Json<PaginateData>) -> actix_web::Result<impl Responder> {
//     let user_list = web::block(move || {
//         let mut conn = pool.get()?;
//         get_source_themes(input.id, input.amount, &mut conn)
//     }).await?
//         .map_err(|_| ApiError::InternalError)?;
//     let user_answer: Vec<SourceThemesResp> = user_list.into_iter().map(|x| x.into()).collect();
//     Ok(HttpResponse::Ok().json(user_answer))
// }


#[get("/sources")]
pub async fn get_sources(pool: Data<DBPool>, query: Query<PaginateData>) -> actix_web::Result<impl Responder> {
    let source_list = web::block(move || {
        let mut conn = pool.get()?;
        get_sources_db(query.id, query.amount, &mut conn)
    }).await?
        .map_err(|_| ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(source_list))
}