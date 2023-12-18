use std::time::Duration;

use actix_cors::Cors;
use actix_rt::time;
use actix_session::{SessionMiddleware, storage::CookieSessionStore, Session, config::PersistentSession};
use actix_web::{web::{self, Data, Json}, App, HttpServer, Responder, get, HttpResponse, middleware::{Logger, Compress, self}, cookie::Key};

use env_logger::Env;
use rust_news_aggregator_v2::{self, db::{establish_connection, DBPool, run_migrations}, api::{auth, self}, background_jobs::start_background_tasks};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pg_conn = establish_connection().expect("Established connection with db");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    run_migrations(&pg_conn).expect("Migrations should have been completed");
    let public_key = Key::generate();
    let task_delay = time::interval(Duration::from_secs(60*5));
    start_background_tasks(pg_conn.clone(),task_delay).await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_conn.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Compress::default())
            // .wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin","*")))
            .wrap(get_cors())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), public_key.clone())
                    .cookie_secure(false)
                    .cookie_content_security(actix_session::config::CookieContentSecurity::Private)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(actix_web::cookie::time::Duration::minutes(10))
                    )
                    .cookie_same_site(actix_web::cookie::SameSite::None)
                    .build()
            )
            .service(hello_world)
            .service(api::api_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// #[cfg(debug_assertions)]
fn get_cors() -> Cors {
    Cors::permissive()
    // Cors::default().allowed_origin("http://192.168.0.4:3000/").supports_credentials()
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
