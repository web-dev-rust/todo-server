use crate::todo_api::model::auth::User;
use crate::todo_api_web::model::auth::SignUp;
use bcrypt::{hash, DEFAULT_COST};

pub fn signup_to_hash_user(su: SignUp) -> User {
    let hashed_pw = hash(su.password, DEFAULT_COST);
    User::from(su.email, hashed_pw.unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::todo_api_web::model::auth::SignUp;
    #[test]
    fn asser_signup_becomes_user() {
        let email = "my@email.com";
        let pass = "this Is a cr4zy p@ssw0rd";
        let signup = SignUp {
            email: String::from(email), 
            password: String::from(pass)
        };
        let user = signup_to_hash_user(signup);
        user.is_user_valid(email, pass)
    }
}
