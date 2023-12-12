use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::{PgConnection, SelectableHelper, QueryDsl, RunQueryDsl, ExpressionMethods};
use anyhow::{Result, Ok};

use crate::{error::ApiError, schema::{roles, users}};

use super::models::*;

pub fn auth_inter(user_form: &UserForm, conn: &mut PgConnection) -> Result<User> {
    use crate::schema::users::dsl::*;
    let user_db = users
        .select(User::as_select())
        .first(conn)?;
    if verify(&user_form.password, &user_db.passwd_hash)? {
        Ok(user_db)
    } else {
        Err(ApiError::LoginError.into())
    }
}

pub fn add_user_inter(user_form: &UserRegister, conn: &mut PgConnection) -> Result<User> {
    // use crate::schema::users::dsl::*;
    let query_role_id: i32 = roles::table
        .filter(roles::name.eq(&user_form.role))
        .select(roles::id)
        .first(conn)?;
    let ret = diesel::insert_into(users::table)
        .values((
            users::email.eq(&user_form.email),
            users::passwd_hash.eq(&hash(&user_form.password, DEFAULT_COST)?),
            users::role_id.eq(query_role_id)
        ))
        .returning(User::as_returning())
        .get_result(conn)?;
    Ok(ret)
    // diesel::insert_into(users)
    //     .
}

#[cfg(test)]
mod tests {
    use crate::db::{establish_connection, models::{UserInsert, UserRegister, UserForm}};

    use super::{add_user_inter, auth_inter};

    #[test]
    fn add_user_to_db() {
        let mut conn = establish_connection().unwrap().get().unwrap();
        let user = UserRegister {
            email: "1".to_string(),
            password: "2".to_string(),
            role: "user".to_string(),
        };
        add_user_inter(&user, &mut conn).unwrap();
    }
    #[test]
    fn get_user() {
        let mut conn = establish_connection().unwrap().get().unwrap();
        let user = UserForm {
            email: "1".to_string(),
            password: "2".to_string(),
        };
        println!("{:?}",auth_inter(&user, &mut conn).unwrap());
    }
}