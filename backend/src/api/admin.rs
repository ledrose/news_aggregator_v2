use std::sync::Arc;

use axum::{extract::{Query, State}, middleware, response::IntoResponse, routing::{get,patch}, Json, Router};
use serde_json::json;
use itertools::Itertools;
use crate::{api::models::{PaginateData, SourceThemePatch, SourceThemesResp, UsersPatch}, auth_middleware, db::{news::news::{delete_sources_db, get_sources_db, insert_sources_db, update_source_themes_db, update_sources_db}, user::user::{delete_users_db, get_all_roles_db, get_source_themes, get_users_db, update_users_db}}, error::ApiError, jwt_middleware, setup::AppState};

use super::models::{UserAnswer, SourcesPatch};

// /admin
pub fn admin_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(get_users).patch(patch_users))
        .route("/sources",get(get_sources).patch(patch_sources))
        .route("/themes",get(get_themes).patch(patch_themes))
        .route("/roles",get(get_all_roles))
        .route_layer(auth_middleware!(state,"admin"))
}


pub async fn get_users(State(state): State<Arc<AppState>>,Query(query): Query<PaginateData>) -> Result<impl IntoResponse,ApiError> {
    let conn = &state.db.get().await.unwrap();
    let user_list = conn.interact(move |conn| {
        get_users_db(query.id0,query.amount,conn)
    }).await??;
    let user_answer: Vec<UserAnswer> = user_list.into_iter().map(|x| x.into()).collect();
    Ok(Json(user_answer))
}

pub async fn get_all_roles(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse,ApiError> {
    let conn = &state.db.get().await.unwrap();
    let roles = conn.interact(move |conn| {
        get_all_roles_db(conn)
    }).await??;
    Ok(Json(roles))
}


pub async fn get_sources(State(state): State<Arc<AppState>>,Query(query): Query<PaginateData>) -> Result<impl IntoResponse,ApiError> {
    let conn = &state.db.get().await.unwrap();
    let source_list = conn.interact(move |conn| {
        get_sources_db(query.id0, query.amount, conn)
    }).await??;
    Ok(Json(source_list))
}

pub async fn patch_sources(State(state): State<Arc<AppState>>,Json(data): Json<Vec<SourcesPatch>>) -> Result<impl IntoResponse,ApiError> {
    println!("{data:?}");
    let conn = &state.db.get().await.unwrap();
    conn.interact( move |conn| {
        let to_update = data.clone().into_iter().filter(|x| x.changed.as_ref().is_some_and(|y| y=="Updated")).map(|x| x.into()).collect_vec();
        let to_add = data.clone().into_iter().filter(|x| x.changed.as_ref().is_some_and(|y| y=="Added")).map(|x| x.into()).collect_vec();
        let to_delete = data.clone().into_iter().filter(|x| x.changed.as_ref().is_some_and(|y| y=="Deleted")).map(|x| x.id).collect_vec();
        if !to_update.is_empty() {
            update_sources_db(to_update, conn)?;
        }
        if !to_add.is_empty() {
            insert_sources_db(to_add, conn)?;
        }
        if !to_delete.is_empty() {
            delete_sources_db(to_delete, conn)?;
        }
        Ok::<(),anyhow::Error>(())
    }).await??;
    Ok(Json(json!({"success":"sucess"})))
}

pub async fn get_themes(State(state): State<Arc<AppState>>,Query(query): Query<PaginateData>) -> Result<impl IntoResponse,ApiError> {
    let conn = &state.db.get().await.unwrap();
    let source_list = conn.interact(move |conn| {
        get_source_themes(query.id0, query.amount,conn)
    }).await??;
    let source_list: Vec<SourceThemesResp> = source_list.into_iter().map(|x| x.into()).collect_vec();
    Ok(Json(source_list))
}

pub async fn patch_themes(State(state): State<Arc<AppState>>,Json(data): Json<Vec<SourceThemePatch>>) -> Result<impl IntoResponse,ApiError> {
    println!("{data:?}");
    let conn = &state.db.get().await.unwrap();
    conn.interact( move |conn| {
        update_source_themes_db(data, conn)
        // get_sources_db(query.id, query.amount, &mut conn)
    }).await??;
    Ok(Json(json!({"success":"sucess"})))
}

pub async fn patch_users(State(state): State<Arc<AppState>>,Json(data): Json<Vec<UsersPatch>>) -> Result<impl IntoResponse,ApiError> {
    println!("{data:?}");
    let conn = &state.db.get().await.unwrap();
    conn.interact(move |conn| {
        let to_update = data.clone().into_iter().filter(|x| x.changed.as_ref().is_some_and(|y| y=="Updated")).map(|x| x.into()).collect_vec();
        let to_delete = data.into_iter().filter(|x| x.changed.as_ref().is_some_and(|y| y=="Deleted")).map(|x| x.id).collect_vec();
        if !to_update.is_empty() {
            update_users_db(to_update, conn)?;
        }
        if !to_delete.is_empty() {
            delete_users_db(to_delete, conn)?;
        }
        Ok::<(),anyhow::Error>(())
        // get_sources_db(query.id, query.amount, &mut conn)
    }).await??;
    Ok(Json(json!({"success":"sucess"})))
}