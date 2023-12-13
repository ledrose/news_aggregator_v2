use actix_web::{web::{self, Data, Json}, App, HttpServer, Responder, get, HttpResponse};

use rust_news_aggregator_v2::{self, db::{establish_connection, DBPool, run_migrations}, api::{auth, self}};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pg_conn = establish_connection().expect("Established connection with db");
    run_migrations(&pg_conn).expect("Migrations should have been completed");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_conn.clone()))
            .service(hello_world)
            .service(api::api_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello_world(pool: Data<DBPool>) -> impl Responder {
    "Hello world"
}
