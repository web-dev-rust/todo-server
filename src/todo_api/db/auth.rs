use diesel::{PgConnection, prelude::*};

use crate::todo_api::model::auth::User;
use crate::todo_api::db::error::DbError;

pub fn insert_new_user(user: User, conn: &PgConnection) -> Result<(),DbError>{
    use crate::schema::auth_user::dsl::*;

    let new_user = diesel::insert_into(auth_user)
        .values(&user)
        .execute(conn);

    match new_user {
        Ok(_) => Ok(()),
        Err(_) => Err(DbError::UserNotCreated)
    }
}

#[cfg(test)]
mod test {
    use diesel::debug_query;
    use diesel::pg::Pg;
    use crate::schema::auth_user::dsl::*;

    #[test]
    fn insert_user_matches_url() {
        use crate::todo_api::model::auth::User;

        let user = User::from(String::from("email@my.com"), String::from("pswd"));
        let query = diesel::insert_into(auth_user).values(&user);
        let sql = String::from("INSERT INTO \"auth_user\" (\"email\", \"id\", \"password\", \"expires_at\") VALUES ($1, $2, $3, $4) \
                -- binds: [\"email@my.com\", ") + &user.id.to_string() + ", \"pswd\", " + &format!("{:?}", user.expires_at) +"]";
        assert_eq!(&sql, &debug_query::<Pg, _>(&query).to_string());
    }
}