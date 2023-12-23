use anyhow::anyhow;
use chrono::Utc;
use diesel::{Queryable, Selectable, associations::{Associations, Identifiable}, prelude::Insertable, query_builder::AsChangeset};
use serde::{Serialize, Deserialize};

use crate::schema::news::description;


#[derive(Selectable, Identifiable, Queryable, Associations,Debug,Serialize,Deserialize,PartialEq)]
#[diesel(table_name = crate::schema::news)]
#[diesel(belongs_to(Source))]
#[diesel(belongs_to(Theme))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEntry {
    pub id: i32,
    pub header: String,
    pub date_time: chrono::DateTime<Utc>,
    pub source_id: i32,
    pub theme_id: i32,
    pub description: Option<String>,
    pub link: String
}

#[derive(Selectable, AsChangeset, Identifiable, Queryable,Debug,Serialize,Deserialize,PartialEq,Eq,Hash)]
#[diesel(table_name = crate::schema::sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub source_type: Option<String>,
    pub link: Option<String>
}

#[derive(Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SourceInsert {
    pub name: String,
    pub source_type: Option<String>,
    pub link: Option<String>
}
 

#[derive(Selectable, Identifiable, Queryable,Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::themes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Theme {
    pub id: i32,
    pub theme_name: String
}

#[derive(Debug,Insertable)]
#[diesel(table_name=crate::schema::themes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ThemeInsert {
    pub theme_name: String
}

impl From<String> for ThemeInsert {
    fn from(value: String) -> Self {
        Self { theme_name: value }
    }
}

#[derive(Selectable, Identifiable, Associations, Queryable,Debug,Serialize,Deserialize,PartialEq, Eq,Hash)]
#[diesel(table_name = crate::schema::sourcethemes)]
#[diesel(belongs_to(Theme))]
#[diesel(belongs_to(Source))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SourceTheme {
    pub id: i32,
    pub source_id: i32,
    pub theme_id: i32,
    pub source_theme_name: String 
}


#[derive(Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::sourcethemes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SourceThemeInsert<'a> {
    pub source_id: i32,
    pub theme_id: i32,
    pub source_theme_name: &'a str 
}

impl<'a> From<(i32,&'a str)> for SourceThemeInsert<'a> {
    fn from(value: (i32,&'a str)) -> Self {
        SourceThemeInsert { source_id: value.0, source_theme_name: value.1, theme_id: 1 }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct NewsFull {
    pub id: i32,
    pub header: String,
    pub date_time: chrono::DateTime<Utc>,
    pub source: String,
    pub theme: String,
    pub description: Option<String>,
    pub link: String
}

#[derive(Debug,Serialize,Deserialize,PartialEq, Eq,Hash)]
pub struct NewsInsert {
    pub header: String,
    pub date_time: chrono::DateTime<Utc>,
    pub source_id: i32,
    pub theme_source: String,
    pub description: Option<String>,
    pub link: String
}

impl<'a> From<(&'a NewsInsert,&'a SourceTheme)> for NewsDBInsert<'a> {
    fn from(value: (&'a NewsInsert,&'a SourceTheme)) -> Self {
        Self { header: &value.0.header, 
            date_time: value.0.date_time, 
            source_id: value.0.source_id, 
            theme_id: value.1.id, 
            link: &value.0.link,
            description: value.0.description.as_deref()
        }
    }
}

#[derive(Insertable, Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::news)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewsDBInsert<'a> {
    pub header: &'a str,
    pub date_time: chrono::DateTime<Utc>,
    pub source_id: i32,
    pub theme_id: i32,
    pub description: Option<&'a str>,
    pub link: &'a str
}

impl TryFrom<(NewEntry,Option<Theme>,Option<Source>)> for NewsFull {
    type Error = anyhow::Error;
 
    fn try_from(value: (NewEntry,Option<Theme>,Option<Source>)) -> Result<Self, Self::Error> {
        Ok(NewsFull {
            id: value.0.id,
            header: value.0.header,
            date_time: value.0.date_time,
            source: value.2.ok_or(anyhow!("Err1"))?.name, 
            theme: value.1.ok_or(anyhow!("Err2"))?.theme_name, 
            link: value.0.link,
            description: value.0.description
        })
    }
}