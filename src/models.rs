use diesel::prelude::*;

#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub passwd_hash: String,
    pub role: i32
}