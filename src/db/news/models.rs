use anyhow::anyhow;
use diesel::{Queryable, Selectable, associations::{Associations, Identifiable}};
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

#[derive(Selectable, Identifiable, Queryable,Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Source {
    pub id: i32,
    pub name: String
}

#[derive(Selectable, Identifiable, Queryable,Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::themes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Theme {
    pub id: i32,
    pub theme_name: String
}

pub struct NewsFull {
    pub id: i32,
    pub header: String,
    pub source: String,
    pub theme: String,
    pub text: String
}

impl From<(NewEntry,Source,Theme)> for NewsFull {
    fn from(value: (NewEntry,Source,Theme)) -> Self {
        NewsFull {
            id: value.0.id,
            header: value.0.header,
            source: value.1.name, 
            theme: value.2.theme_name, 
            text: value.0.text 
        }
    }
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