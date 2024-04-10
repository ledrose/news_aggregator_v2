use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::db::{feeds::models::{Feed, FeedSource}, news::models::{NewsFull, Source, SourceInsert, SourceTheme, Theme}, user::models::{Role, User, UserUpdate}};

#[derive(Serialize,Deserialize,Debug)]
pub struct NewsBatchInfo {
    pub max_id: Option<i32>,
    pub offset: i64,
    pub amount: i64,
    pub prefs: SearchQuery
}

#[derive(Serialize,Deserialize,Debug)]
pub struct NewsReturn {
    pub max_id: i32,
    pub news: Vec<NewsFull>
}

#[derive(Debug,Serialize,)]
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

#[serde_as]
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SearchQuery { 
    pub query: Option<String>,
    pub allowed_sources: Vec<String>,
    pub add_source: Vec<String>,
    pub remove_source: Vec<String>,
    pub add_themes: Vec<String>,
    pub remove_themes: Vec<String>,
    #[serde_as(as="Option<serde_with::TimestampSeconds<i64>>")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde_as(as="Option<serde_with::TimestampSeconds<i64>>")]
    pub end_date: Option<DateTime<Utc>>,
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

#[derive(Debug,Deserialize,Serialize)]

pub struct FeedInfo {
    pub name: String,
    pub sources: Vec<String>
}

// string is sources
impl From<(Vec<(FeedSource,String)>,Feed)> for FeedInfo {
    fn from(value: (Vec<(FeedSource,String)>,Feed)) -> Self {
        let feed_name = value.1.name;
        let vec = value.0.into_iter()
            .map(|x| x.1)
            .collect_vec();
        Self { name: feed_name, sources: vec }
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
