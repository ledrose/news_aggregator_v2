
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable,Selectable,Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub passwd_hash: String,
    pub role_id: i32
}

#[derive(Insertable,Debug, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInsert<'a> {
    pub email: &'a str,
    pub passwd_hash: &'a str,
    pub role_id: i32
}
#[derive(Debug,Deserialize)]
pub struct UserForm {
    pub email: String,
    pub password: String
}
#[derive(Debug,Deserialize)]
pub struct UserRegister {
    pub email: String,
    pub password: String,
    pub role: String
}
