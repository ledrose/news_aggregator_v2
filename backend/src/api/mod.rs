use actix_web::{web::{ServiceConfig, self}, Scope};


pub mod auth;
pub mod news;
pub mod admin;
pub mod models;

pub fn api_scope() -> Scope {
    Scope::new("/api")
        .service(auth::auth_scope())
        .service(news::news_scope())
        .service(admin::admin_scope())
}
