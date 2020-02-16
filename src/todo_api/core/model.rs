use actix::prelude::*;
use crate::todo_api::{
    db::{
        error::DbError,
        helpers::DbExecutor,
    },
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UpdateDate {
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl Message for UpdateDate {
    type Result = Result<(), DbError>;
}

impl Handler<UpdateDate> for DbExecutor {
    type Result = Result<(), DbError>;

    fn handle(&mut self, msg: UpdateDate, _: &mut Self::Context) -> Self::Result {
        use crate::todo_api::db::auth::update_user_jwt_date;

        update_user_jwt_date(msg, &self.0.get().expect("Failed to open connection"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Jwt{
    token: String
}

impl Jwt {
    pub fn new(jwt: String) -> Self {
        Self {
            token: jwt
        }
    }
}