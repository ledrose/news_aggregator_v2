use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::{user::models::{User, Role}, news::models::{Theme, Source, SourceTheme}};

#[derive(Serialize,Deserialize,Debug)]
pub struct NewsBatchInfo {
    pub start_date: Option<DateTime<Utc>>,
    pub amount: i64,
    pub prefs: SearchQuery
}

#[derive(Debug,Serialize)]
pub struct UserAnswer {
    pub email: String,
    pub role: String,
}

impl From<(User,Role)> for UserAnswer {
    fn from(value: (User,Role)) -> Self {
        Self { email: value.0.email, role: value.1.name }
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
    pub id: Option<i32>,
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
