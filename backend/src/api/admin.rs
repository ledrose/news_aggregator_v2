use actix_session::SessionExt;
use actix_web::{Scope, guard, web, post, Responder, HttpResponse};
use serde_json::json;


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

#[post("/users")]
async fn get_user_list() -> impl Responder {
    HttpResponse::Ok().json(json!({"answer":"You are logged"}))
}