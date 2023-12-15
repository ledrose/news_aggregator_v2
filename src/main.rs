use actix_session::{SessionMiddleware, storage::CookieSessionStore, Session, config::PersistentSession};
use actix_web::{web::{self, Data, Json}, App, HttpServer, Responder, get, HttpResponse, middleware::Logger, cookie::Key};

use env_logger::Env;
use rust_news_aggregator_v2::{self, db::{establish_connection, DBPool, run_migrations}, api::{auth, self}};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pg_conn = establish_connection().expect("Established connection with db");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    run_migrations(&pg_conn).expect("Migrations should have been completed");
    let public_key = Key::generate();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_conn.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), public_key.clone())
                    .cookie_secure(false)
                    .cookie_content_security(actix_session::config::CookieContentSecurity::Private)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(actix_web::cookie::time::Duration::minutes(10))
                    )
                    .build()
            )
            .service(hello_world)
            .service(api::api_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello_world(pool: Data<DBPool>, session: Session) -> actix_web::Result<impl Responder> {
    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count+1)?;
    } else {
        session.insert("counter", 1)?;
    }
    // log::debug!("Hello world message");
    Ok(HttpResponse::Ok().body(format!("Count is {:?}",session.get::<i32>("counter")?.unwrap())))
}
