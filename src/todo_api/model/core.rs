use crate::todo_api::{db::helpers::DbExecutor, model::error::DbError};
use actix::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UpdateUserStatus {
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
    pub is_active: bool,
}

impl Message for UpdateUserStatus {
    type Result = Result<(), DbError>;
}

impl Handler<UpdateUserStatus> for DbExecutor {
    type Result = Result<(), DbError>;

    fn handle(&mut self, msg: UpdateUserStatus, _: &mut Self::Context) -> Self::Result {
        use crate::todo_api::db::auth::update_user_jwt_date;

        update_user_jwt_date(msg, &self.0.get().expect("Failed to open connection"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Jwt {
    token: String,
}

impl Jwt {
    pub fn new(jwt: String) -> Self {
        Self { token: jwt }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct JwtValue {
    pub id: String,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Inactivate {
    pub email: String,
    pub is_active: bool,
}

impl Inactivate {
    pub fn new(email: String) -> Self {
        Self {
            email: email,
            is_active: false,
        }
    }
}

impl Message for Inactivate {
    type Result = Result<(), DbError>;
}

impl Handler<Inactivate> for DbExecutor {
    type Result = Result<(), DbError>;

    fn handle(&mut self, msg: Inactivate, _: &mut Self::Context) -> Self::Result {
        use crate::todo_api::db::auth::inactivate_user;

        inactivate_user(msg, &self.0.get().expect("Failed to open connection"))
    }
}
