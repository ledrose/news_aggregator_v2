
use diesel::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Queryable,Selectable,Debug,Serialize,Deserialize,Clone)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: i32,
    pub name: String
}
#[derive(Debug,Serialize,Deserialize)]
pub enum RoleEnum {
    User,
    Admin
}

#[derive(Queryable,Associations,Selectable,Debug,Serialize,Deserialize,Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(belongs_to(Role))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub passwd_hash: String,
    pub role_id: i32
}
#[derive(Debug,Serialize,Deserialize)]
pub struct UserWithRole {
    pub id: i32,
    pub email: String,
    pub role: String
}

pub struct UserUpdate {
    pub id: i32,
    pub role: String,
}

#[derive(Debug,Deserialize)]
pub struct UserForm {
    pub email: String,
    pub password: String
}
#[derive(Debug,Deserialize,Clone)]
pub struct UserRegister {
    pub email: String,
    pub password: String,
    // pub role: String
}
