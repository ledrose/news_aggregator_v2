use actix_web::{web::{ServiceConfig, self}, Scope};


pub mod auth;
pub mod news;

pub fn api_scope() -> Scope {
    Scope::new("/api")
        .service(auth::auth_scope())
        .service(news::news_scope())
}