use actix_session::SessionExt;
use actix_web::{Scope, guard::{self, GuardContext, Guard}, web::{self, Data, Json, Query, route}, post, Responder, HttpResponse, get, dev::Service};

use crate::{db::{user::user::{get_all_users, get_source_themes}, DBPool, news::news::get_sources_db}, error::ApiError, api::models::{PaginateData, SourceThemesResp}};

use super::models::UserAnswer;


pub fn admin_scope() -> Scope {
    web::scope("/admin")
        .service(get_sources)
        .default_service(web::route().to(HttpResponse::Forbidden))
        // .service(web::h)
    
    // web::resource("/admin")
    //     .route(
    //         web::route().guard(guard::fn_guard(admin_guard)).service(get_sources)
    //     ).route(
        
    //     )
    //         .service(get_user_list)
        
}

// struct AdminGuard;
// impl Guard for AdminGuard {
//     fn check(&self, ctx: &GuardContext<'_>) -> bool {
//         println!("In guardCheck");
//         if let Ok(Some(role)) = ctx.get_session().get::<String>("role") {
//             println!("RoleGuard: {role}");
//             if role == "admin" {
//                 println!("GuardCheck True");
//                 return true;
//             }
//         }
//         println!("GuardCheck False");
//         false
//     }
// }

fn admin_guard(ctx: &GuardContext<'_>) -> bool {
    println!("In guardCheck");
    if let Ok(Some(role)) = ctx.get_session().get::<String>("role") {
        println!("RoleGuard: {role}");
        if role == "admin" {
            println!("GuardCheck True");
            return true;
        }
    }
    println!("GuardCheck False");
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


#[get("/sources")]
pub async fn get_sources(pool: Data<DBPool>, query: Query<PaginateData>) -> actix_web::Result<impl Responder> {
    let source_list = web::block(move || {
        let mut conn = pool.get()?;
        get_sources_db(query.id, query.amount, &mut conn)
    }).await?
        .map_err(|_| ApiError::InternalError)?;
    Ok(HttpResponse::Ok().json(source_list))
}