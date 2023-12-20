use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::user::models::{User, Role};

#[derive(Serialize,Deserialize,Debug)]
pub struct NewsBatchInfo {
    pub start_date: Option<DateTime<Utc>>,
    pub amount: i64
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
