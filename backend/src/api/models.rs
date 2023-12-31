use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::{user::models::{User, Role, UserUpdate}, news::models::{Theme, Source, SourceTheme, SourceInsert}};

#[derive(Serialize,Deserialize,Debug)]
pub struct NewsBatchInfo {
    pub start_date: Option<DateTime<Utc>>,
    pub amount: i64,
    pub prefs: SearchQuery
}

#[derive(Debug,Serialize)]
pub struct UserAnswer {
    pub id: i32,
    pub email: String,
    pub role: String,
}

impl From<(User,Role)> for UserAnswer {
    fn from(value: (User,Role)) -> Self {
        Self { id: value.0.id, email: value.0.email, role: value.1.name }
    }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SearchQuery { 
    pub query: Option<String>,
    pub add_source: Vec<String>,
    pub remove_source: Vec<String>,
    pub add_themes: Vec<String>,
    pub remove_themes: Vec<String>
}

#[derive(Debug,Deserialize)]
pub struct PaginateData {
    pub id0: Option<i32>,
    pub amount: i64
}

#[derive(Debug,Serialize)]
pub struct SourceThemesResp {
    pub id: i32,
    pub source: String,
    pub theme: String,
    pub name: String
}

impl From<(SourceTheme,Theme,Source)> for SourceThemesResp {
    fn from(value: (SourceTheme,Theme,Source)) -> Self {
        Self { id: value.0.id, source: value.2.name, theme: value.1.theme_name, name: value.0.source_theme_name }
    }
}

#[derive(Debug,Deserialize,Clone)]
pub struct SourcesPatch {
    pub id: i32,
    pub name: String,
    pub source_type: String,
    pub link: String,
    pub changed: Option<String>
}

impl From<SourcesPatch> for Source {
    fn from(value: SourcesPatch) -> Self {
        let SourcesPatch { id, name, source_type, link, .. } = value;
        Self { id, name, source_type: Some(source_type), link: Some(link) }
    }
}

impl From<SourcesPatch> for SourceInsert {
    fn from(value: SourcesPatch) -> Self {
        let SourcesPatch {name, source_type, link, .. } = value;
        Self { name, source_type: Some(source_type), link: Some(link) }
    }
}

#[derive(Debug,Serialize)]
pub struct SearchOptions {
    pub sources: Vec<String>,
    pub themes: Vec<String>
}

#[derive(Debug,Deserialize,Clone)]
pub struct SourceThemePatch {
    pub id: i32,
    pub theme: String,
}

#[derive(Debug,Deserialize,Clone)]
pub struct UsersPatch {
    pub id: i32,
    pub role: String,
    pub changed: Option<String>
}

impl From<UsersPatch> for UserUpdate {
    fn from(value: UsersPatch) -> Self {
        let UsersPatch { id, role, .. } = value;
        Self { id, role }
    }
}