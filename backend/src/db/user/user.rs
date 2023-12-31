use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::{PgConnection, SelectableHelper, QueryDsl, RunQueryDsl, ExpressionMethods};
use anyhow::{Result, Ok, anyhow};
use itertools::Itertools;

use crate::{error::ApiError, schema::{roles, users, sourcethemes, sources, themes}, db::news::models::{SourceTheme, Source, Theme}};

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

pub fn add_user_inter(user_form: &UserRegister, conn: &mut PgConnection) -> Result<User,ApiError> {
    use diesel::OptionalExtension;
    let res: Option<User> = users::table
        .filter(users::email.eq(&user_form.email))
        .first(conn)
        .optional().map_err(|_| ApiError::InternalError)?;
    if res.is_some() {
        Err(ApiError::RegistrationError.into())
    } else {
        let ret = diesel::insert_into(users::table)
        .values((
            users::email.eq(&user_form.email),
            users::passwd_hash.eq(&hash(&user_form.password, DEFAULT_COST).map_err(|_| ApiError::InternalError)?),
            // users::role_id.eq(query_role_id)
        ))
        .returning(User::as_returning())
        .get_result(conn).map_err(|_| ApiError::InternalError)?;
        std::prelude::rust_2021::Ok(ret)
    }
}


pub fn get_all_roles_db(conn: &mut PgConnection) -> Result<Vec<Role>> {
    let ret = roles::table
        .select(Role::as_select())
        .get_results(conn)?;
    Ok(ret)
}

pub fn get_users_db(id: Option<i32>, amount: i64, conn: &mut PgConnection) -> Result<Vec<(User,Role)>> {
    let id = id.unwrap_or(0);
    let res = users::table
        .inner_join(roles::table)
        .filter(users::id.ge(id))
        .limit(amount)
        .select((User::as_select(), Role::as_select()))
        .get_results::<(User,Role)>(conn)?;
    Ok(res)
}


pub fn update_users_db(users_vec: Vec<UserUpdate>,conn: &mut PgConnection) -> Result<(),anyhow::Error> {
    let roles = get_all_roles_db(conn)?;
    let users_vec = users_vec.iter().map(|x| (x,roles.iter().find_or_first(|y| x.role==y.name)))
        .filter(|x| x.1.is_some())
        .map(|x| (x.0.id,x.1.unwrap().id)).collect_vec();
    for user in users_vec {
        diesel::update(users::table)
            .set(users::role_id.eq(user.1))
            .filter(users::role_id.eq(user.0))
            .execute(conn)?;
    }
    Ok(())
}
pub fn delete_users_db(ids: Vec<i32>,conn: &mut PgConnection) -> Result<(),anyhow::Error> {
    diesel::delete(users::table)
        .filter(users::id.eq_any(ids))
        .execute(conn)?;
    Ok(())
}

pub fn get_role_db(email: &str, conn: &mut PgConnection) -> Result<Role> {
    let role: Role = users::table
        .inner_join(roles::table)
        .filter(users::email.eq(email))
        .select(Role::as_select())
        .first::<Role>(conn)?;
    Ok(role)
}

pub fn get_source_themes(id: Option<i32>, amount: i64, conn: &mut PgConnection) -> Result<Vec<(SourceTheme,Theme,Source)>> {
    let id = id.unwrap_or(0);
    let res = sourcethemes::table
        .inner_join(themes::table)
        .inner_join(sources::table)
        .filter(sourcethemes::id.ge(id))
        .order_by(sourcethemes::id.asc())
        .limit(amount)
        .select((SourceTheme::as_select(),Theme::as_select(),Source::as_select()))
        .get_results::<(SourceTheme,Theme,Source)>(conn)?;
    Ok(res)
}

pub fn get_sources(id: i32,amount: i64, conn: &mut PgConnection) -> Result<Vec<Source>> {
    let res = sources::table
        .filter(sources::id.ge(id))
        .order_by(sources::id.asc())
        .limit(amount)
        .select(Source::as_select())
        .get_results::<Source>(conn)?;
    Ok(res)
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