use actix_session::SessionExt;
use actix_web::{Scope, guard::{self, GuardContext, Guard}, web::{self, Data, Json, Query, route}, post, Responder, HttpResponse, get, dev::Service, patch};use itertools::Itertools;
use serde_json::json;

use crate::{db::{user::user::{get_all_users, get_source_themes}, DBPool, news::news::{get_sources_db, update_sources_db, insert_sources_db, delete_sources_db}}, error::ApiError, api::models::{PaginateData, SourceThemesResp}};

use super::models::{UserAnswer, SourcesPatch};


pub fn admin_scope() -> Scope {
    web::scope("/admin")
        .service(get_sources)
        .service(patch_sources)
        .default_service(web::route().to(HttpResponse::Forbidden))
}
 
fn admin_guard(ctx: &GuardContext<'_>) -> bool {
    if let Ok(Some(role)) = ctx.get_session().get::<String>("role") {
        if role == "admin" {
            println!("GuardCheck True");
            return true;
        }
    }
    false
}


#[get("/users",guard="admin_guard")]
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


#[get("/sources",guard="admin_guard")]
pub async fn get_sources(pool: Data<DBPool>, query: Query<PaginateData>) -> actix_web::Result<impl Responder> {
    let source_list = web::block(move || {
        let mut conn = pool.get()?;
        get_sources_db(query.id, query.amount, &mut conn)
    }).await?
        .map_err(|_| ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(source_list))
}

#[patch("/sources",guard="admin_guard")]
pub async fn patch_sources(pool: Data<DBPool>,data: Json<Vec<SourcesPatch>>) -> actix_web::Result<impl Responder> {
    println!("{data:?}");
    web::block(move || {
        let mut conn = pool.get()?;
        let to_update = data.0.clone().into_iter().filter(|x| x.changed.as_ref().is_some_and(|y| y=="Updated")).map(|x| x.into()).collect_vec();
        let to_add = data.0.clone().into_iter().filter(|x| x.changed.as_ref().is_some_and(|y| y=="Added")).map(|x| x.into()).collect_vec();
        let to_delete = data.0.clone().into_iter().filter(|x| x.changed.as_ref().is_some_and(|y| y=="Deleted")).map(|x| x.id).collect_vec();
        update_sources_db(to_update, &mut conn)?;
        insert_sources_db(to_add, &mut conn)?;
        delete_sources_db(to_delete, &mut conn)
        // get_sources_db(query.id, query.amount, &mut conn)
    }).await?
        .map_err(|_| ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(json!({"success":"sucess"})))
}