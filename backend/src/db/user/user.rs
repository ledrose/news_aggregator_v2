use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::{PgConnection, SelectableHelper, QueryDsl, RunQueryDsl, ExpressionMethods};
use anyhow::{Result, Ok};

use crate::{error::ApiError, schema::{roles, users}};

use super::models::*;

pub fn auth_inter(user_form: &UserForm, conn: &mut PgConnection) -> Result<UserWithRole> {
    // use crate::schema::users::dsl::*;
    let user_db: (User,Role) = users::table
        .inner_join(roles::table)
        .filter(users::email.eq(&user_form.email))
        .select((User::as_select(),Role::as_select()))
        .first(conn)?;
    if verify(&user_form.password, &user_db.0.passwd_hash)? {
        Ok(UserWithRole { id: user_db.0.id, email: user_db.0.email, role: user_db.1.name })
    } else {
        Err(ApiError::LoginError.into())
    }
}

pub fn add_user_inter(user_form: &UserRegister, conn: &mut PgConnection) -> Result<User> {
    let ret = diesel::insert_into(users::table)
        .values((
            users::email.eq(&user_form.email),
            users::passwd_hash.eq(&hash(&user_form.password, DEFAULT_COST)?),
            // users::role_id.eq(query_role_id)
        ))
        .returning(User::as_returning())
        .get_result(conn)?;
    Ok(ret)
}

pub fn get_all_users(conn: &mut PgConnection) -> Result<Vec<(User,Role)>> {
    let res = users::table
        .inner_join(roles::table)
        .select((User::as_select(), Role::as_select()))
        .get_results::<(User,Role)>(conn)?;
    Ok(res)
}

pub fn get_role_db(email: &str, conn: &mut PgConnection) -> Result<Role> {
    let role: Role = users::table
        .inner_join(roles::table)
        .filter(users::email.eq(email))
        .select(Role::as_select())
        .first::<Role>(conn)?;
    Ok(role)
}

#[cfg(test)]
mod tests {
    use crate::db::{establish_connection, user::models::{UserRegister, UserForm}};

    use super::{add_user_inter, auth_inter};

    #[test]
    fn add_user_to_db() {
        let mut conn = establish_connection().unwrap().get().unwrap();
        let user = UserRegister {
            email: "1".to_string(),
            password: "2".to_string()
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