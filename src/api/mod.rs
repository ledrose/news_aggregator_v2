use actix_web::{web::{ServiceConfig, self}, Scope};

pub mod auth;

pub fn api_scope() -> Scope {
    Scope::new("/api")
        .service(auth::auth_scope())
}