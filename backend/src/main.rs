
use std::{sync::Arc, time::Duration};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use rust_news_aggregator_v2::{self, api::{self, api_router}, background_jobs::start_background_tasks, setup::establish_connection};
use tokio::time;
use tower_http::{cors::CorsLayer, services::{ServeDir, ServeFile}, trace::TraceLayer};


#[tokio::main]
async fn main() {
    init_tracing();
    let state = Arc::new(establish_connection().await);
    let task_delay = time::interval(Duration::from_secs(60*5));
    start_background_tasks(state.db.clone(),task_delay).await;
    let app = Router::new()
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .me("/api-docs/openapi.json", get(openapi))
        .nest("/api", api_router(state.clone()))
        .route_service("/", ServeDir::new("build"))
        .route_service("/*key", ServeDir::new("build").fallback(ServeFile::new("build/index.html")))
        .fallback(err_404)
        .layer(CorsLayer::very_permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {}",&addr);
    axum::serve(listener,app).await.unwrap();
}
async fn err_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND,"The requested resource was not found")
}


fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}