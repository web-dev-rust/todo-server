use diesel::{prelude::*, PgConnection};

use crate::todo_api::model::auth::User;
use crate::todo_api::model::{
    core::{Inactivate, UpdateUserStatus, JwtValue},
    error::DbError,
};

#[cfg(not(feature = "dbtest"))]
pub fn insert_new_user(user: User, conn: &PgConnection) -> Result<(), DbError> {
    use crate::schema::auth_user::dsl::*;

    let new_user = diesel::insert_into(auth_user).values(&user).execute(conn);

    match new_user {
        Ok(_) => Ok(()),
        Err(_) => Err(DbError::UserNotCreated),
    }
}

#[cfg(feature = "dbtest")]
pub fn insert_new_user(_user: User, _: &PgConnection) -> Result<(), DbError> {
    use crate::schema::auth_user::dsl::*;
    use diesel::debug_query;
    use diesel::pg::Pg;

    let user = User::from(
        String::from("my@email.com"),
        String::from("My cr4azy p@ssw0rd My cr4azy p@ssw0rd"),
    );
    let query = diesel::insert_into(auth_user).values(&user);
    let sql = "INSERT INTO \"auth_user\" (\"email\", \"id\", \"password\", \"expires_at\", \"is_active\") VALUES ($1, $2, $3, $4, $5) \
            -- binds: [\"my@email.com\", ";
    assert!(debug_query::<Pg, _>(&query).to_string().contains(sql));
    assert!(debug_query::<Pg, _>(&query)
        .to_string()
        .contains("My cr4azy p@ssw0rd My cr4azy p@ssw0rd"));

    Ok(())
}

#[cfg(not(feature = "dbtest"))]
pub fn scan_user(user_email: String, conn: &PgConnection) -> Result<User, DbError> {
    use crate::schema::auth_user::dsl::*;

    let items = auth_user.filter(email.eq(&user_email)).load::<User>(conn);

    match items {
        Ok(users) if users.len() > 1 => Err(DbError::DatabaseConflit),
        Ok(users) if users.len() < 1 => Err(DbError::CannotFindUser),
        Ok(users) => Ok(users.first().unwrap().clone().to_owned()),
        Err(_) => Err(DbError::CannotFindUser),
    }
}

#[cfg(feature = "dbtest")]
pub fn scan_user(user_email: String, _conn: &PgConnection) -> Result<User, DbError> {
    Ok(User::from(user_email, "this is a hash".to_string()))
}

#[cfg(feature = "dbtest")]
pub fn test_scan_user(user_email: String, id: String, _conn: &PgConnection) -> Result<User, DbError> {
    Ok(User::test_from(user_email, "this is a hash".to_string(), id))
}

#[cfg(not(feature = "dbtest"))]
pub fn update_user_jwt_date(
    update_date: UpdateUserStatus,
    conn: &PgConnection,
) -> Result<(), DbError> {
    use crate::schema::auth_user::dsl::*;

    let target = auth_user.filter(email.eq(update_date.email));
    match diesel::update(target)
        .set((
            expires_at.eq(update_date.expires_at),
            is_active.eq(update_date.is_active),
        ))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(_) => Err(DbError::TryAgain),
    }
}

#[cfg(feature = "dbtest")]
pub fn update_user_jwt_date(
    _update_date: UpdateUserStatus,
    _conn: &PgConnection,
) -> Result<(), DbError> {
    Ok(())
}

#[cfg(not(feature = "dbtest"))]
pub fn inactivate_user(msg: Inactivate, conn: &PgConnection) -> Result<(), DbError> {
    use crate::schema::auth_user::dsl::*;

    let target = auth_user.filter(email.eq(msg.email));
    match diesel::update(target)
        .set(is_active.eq(msg.is_active))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(_) => Err(DbError::TryAgain),
    }
}

#[cfg(feature = "dbtest")]
pub fn inactivate_user(_msg: Inactivate, _conn: &PgConnection) -> Result<(), DbError> {
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::schema::auth_user::dsl::*;
    use diesel::debug_query;
    use diesel::pg::Pg;

    #[test]
    fn insert_user_matches_url() {
        use crate::todo_api::model::auth::User;

        let user = User::from(String::from("email@my.com"), String::from("pswd"));
        let query = diesel::insert_into(auth_user).values(&user);
        let sql = String::from("INSERT INTO \"auth_user\" (\"email\", \"id\", \"password\", \"expires_at\", \"is_active\") VALUES ($1, $2, $3, $4, $5) \
                -- binds: [\"email@my.com\", ") + &user.id.to_string() + ", \"pswd\", " + &format!("{:?}", user.expires_at) +", false]";
        assert_eq!(&sql, &debug_query::<Pg, _>(&query).to_string());
    }
}
