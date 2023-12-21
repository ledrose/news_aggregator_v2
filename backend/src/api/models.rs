use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::user::models::{User, Role};

#[derive(Serialize,Deserialize,Debug)]
pub struct NewsBatchInfo {
    pub start_date: Option<DateTime<Utc>>,
    pub amount: i64,
    pub prefs: Option<Vec<Preferences>>
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
pub struct Preferences { 
    pub action: PreferenceAction,
    pub pref_type: PreferenceType
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum PreferenceAction {
    Add,
    Remove
}
#[derive(Debug,Serialize,Deserialize,Clone)]
#[serde(tag="type")]
pub enum PreferenceType {
    Source {name:String},
    Theme {name:String}
}

