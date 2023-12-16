use anyhow::anyhow;
use diesel::{Queryable, Selectable, associations::{Associations, Identifiable}, prelude::Insertable};
use serde::{Serialize, Deserialize};


#[derive(Selectable, Identifiable, Queryable, Associations,Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::news)]
#[diesel(belongs_to(Source))]
#[diesel(belongs_to(Theme))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEntry {
    pub id: i32,
    pub header: String,
    pub source_id: i32,
    pub theme_id: i32,
    pub text: String
}

#[derive(Selectable, Identifiable, Queryable,Debug,Serialize,Deserialize,PartialEq,Eq,Hash)]
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
pub struct SourceInsert<'a> {
    pub name: &'a str,
}

impl<'a> From<&'a str> for SourceInsert<'a> {
    fn from(value: &'a str) -> Self {
        Self { name: value }
    }
} 

#[derive(Selectable, Identifiable, Queryable,Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::themes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Theme {
    pub id: i32,
    pub theme_name: String
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
    pub source: String,
    pub theme: String,
    pub text: String
}

#[derive(Debug,Serialize,Deserialize,PartialEq, Eq,Hash)]
pub struct NewsInsert {
    pub header: String,
    pub source_id: i32,
    pub theme_source: String,
    pub text: String
}

impl<'a> From<(&'a NewsInsert,&'a SourceTheme)> for NewsDBInsert<'a> {
    fn from(value: (&'a NewsInsert,&'a SourceTheme)) -> Self {
        Self { header: &value.0.header, source_id: value.0.source_id, theme_id: value.1.id, text: &value.0.text }
    }
}

#[derive(Insertable, Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::news)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewsDBInsert<'a> {
    pub header: &'a str,
    pub source_id: i32,
    pub theme_id: i32,
    pub text: &'a str
}

impl TryFrom<(NewEntry,Option<Theme>,Option<Source>)> for NewsFull {
    type Error = anyhow::Error;
 
    fn try_from(value: (NewEntry,Option<Theme>,Option<Source>)) -> Result<Self, Self::Error> {
        Ok(NewsFull {
            id: value.0.id,
            header: value.0.header,
            source: value.2.ok_or(anyhow!("Err1"))?.name, 
            theme: value.1.ok_or(anyhow!("Err2"))?.theme_name, 
            text: value.0.text 
        })
    }
}