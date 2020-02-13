use crate::todo_api::model::auth::User;
use crate::todo_api_web::model::auth::SignUp;
use bcrypt::{hash, DEFAULT_COST};

pub fn signup_to_hash_user(su: SignUp) -> User {
    let hashed_pw = hash(su.password, DEFAULT_COST);
    User::from(su.email, hashed_pw.unwrap())
}
