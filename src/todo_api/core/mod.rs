use actix_web::{web, HttpResponse};
use log::error;
use serde_json::value::Value;

use crate::todo_api::model::{
    auth::User,
    core::{Inactivate, UpdateUserStatus},
    error::DbError,
};
use crate::todo_api_web::model::http::Clients;

pub async fn generate_jwt(user: User, state: web::Data<Clients>) -> HttpResponse {
    let utc = crate::todo_api::db::helpers::one_day_from_now().naive_utc();

    let update_date = UpdateUserStatus {
        email: user.email.clone(),
        expires_at: utc,
        is_active: true,
    };

    let resp = state.postgres.send(update_date.clone()).await;

    match resp {
        Ok(_) => {
            let token_jwt = create_token(user, update_date);
            let jwt = crate::todo_api::model::core::Jwt::new(token_jwt);
            HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&jwt).unwrap())
        }
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn create_token(user: User, update_date: UpdateUserStatus) -> String {
    use chrono::Utc;
    use jsonwebtokens::{encode, Algorithm, AlgorithmID};
    use serde_json::json;

    let alg = Algorithm::new_hmac(AlgorithmID::HS256, "secret").unwrap();
    let header = json!({ "alg": alg.name(), "typ": "jwt", "date":  Utc::now().to_string()});
    let payload = json!({ "id": user.clone().get_id(), "email": user.email, "expires_at": update_date.expires_at });
    encode(&header, &payload, &alg).unwrap()
}

#[cfg(not(feature = "dbtest"))]
pub fn decode_jwt(jwt: &str) -> Value {
    use jsonwebtokens::raw::{decode_json_token_slice, split_token, TokenSlices};

    let TokenSlices { claims, .. } = split_token(jwt).unwrap();
    let claims = decode_json_token_slice(claims).expect("Failed to decode token");
    claims
}

#[cfg(feature = "dbtest")]
pub fn decode_jwt(jwt: &str) -> Value {
    serde_json::from_str("{\"expires_at\": \"3020-11-02T00:00:00\", \"id\": \"bc45a88e-8bb9-4308-a206-6cc6eec9e6a1\", \"email\": \"my@email.com\"}").unwrap()
}

pub fn validate_jwt_date(jwt_expires: chrono::NaiveDateTime) -> bool {
    chrono::Utc::now().naive_utc() <= jwt_expires
}

pub async fn validate_jwt_info(
    jwt_email: String,
    req_email: String,
    user: Result<User, DbError>,
    state: web::Data<Clients>,
) -> HttpResponse {
    match user {
        Err(_) => HttpResponse::Unauthorized().finish(),
        Ok(u) => {
            if u.email == jwt_email && jwt_email == req_email {
                let inactivate = Inactivate::new(req_email);
                let is_inactive = state.postgres.send(inactivate).await;

                match is_inactive {
                    Ok(_) => HttpResponse::Accepted().finish(),
                    Err(_) => HttpResponse::Unauthorized().finish(),
                }
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
    }
}

pub fn is_email_pswd_valids(email: &str, pswd: &str) -> bool {
    use regex::Regex;

    let email_regex = Regex::new("\\w{1,}@\\w{2,}.[a-z]{2,3}(.[a-z]{2,3})?$").unwrap();
    let pswd_regex = Regex::new("[[a-z]+[A-Z]+[0-9]+(\\s@!=_#&~\\[\\]\\{\\}\\?)]{32,64}").unwrap();

    email_regex.is_match(email) && pswd_regex.is_match(pswd)
}

#[cfg(test)]
mod jwt_validations {
    use super::{validate_jwt_date, validate_jwt_info};
    use crate::todo_api::model::auth::User;
    use crate::todo_api_web::model::http::Clients;
    use actix_web::http::StatusCode;
    use chrono::{DateTime, Utc};

    #[test]
    fn date_smaller_than_now_is_false() {
        let expires = "2014-11-28T12:00:09Z".parse::<DateTime<Utc>>().unwrap();
        assert!(!validate_jwt_date(expires.naive_utc()));
    }

    #[test]
    fn date_way_larger_than_now_is_true() {
        let expires = "3014-11-28T12:00:09Z".parse::<DateTime<Utc>>().unwrap();
        assert!(validate_jwt_date(expires.naive_utc()));
    }

    #[actix_rt::test]
    async fn all_args_are_equal_is_accepted() {
        use dotenv::dotenv;
        dotenv().ok();

        let exec = Clients::new();
        let state = actix_web::web::Data::new(exec);

        let user = User::from("my@email.com".to_string(), "pass".to_string());
        let email = "my@email.com".to_string();

        let resp = validate_jwt_info(email.clone(), email, Ok(user), state).await;
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }

    #[actix_rt::test]
    async fn all_args_are_not_equal_is_unauth() {
        use dotenv::dotenv;
        dotenv().ok();

        let exec = Clients::new();
        let state = actix_web::web::Data::new(exec);

        let user = User::from("not_my@email.com".to_string(), "pass".to_string());
        let email = "my@email.com".to_string();

        let resp = validate_jwt_info(email.clone(), email, Ok(user), state).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}

#[cfg(test)]
mod valid_email_pswd {
    use super::is_email_pswd_valids;

    #[test]
    fn valid_email_and_pswd() {
        assert!(is_email_pswd_valids(
            "my@email.com",
            "My cr4zy P@ssw0rd My cr4zy P@ssw0rd"
        ));
    }

    #[test]
    fn invalid_emails() {
        assert!(!is_email_pswd_valids(
            "my_email.com",
            "My cr4zy P@ssw0rd My cr4zy P@ssw0rd"
        ));
        assert!(!is_email_pswd_valids(
            "my@email.com.br.us",
            "My cr4zy P@ssw0rd My cr4zy P@ssw0rd"
        ));
    }

    #[test]
    fn invalid_passwords() {
        assert!(!is_email_pswd_valids(
            "my@email.com.br",
            "My cr4zy P@ssw0rd"
        ));
        assert!(is_email_pswd_valids(
            "my@email.com",
            "my cr4zy p@ssw0rd my cr4zy p@ssw0rd"
        ));
        assert!(is_email_pswd_valids(
            "my@email.com",
            "My crazy P@ssword My crazy P@ssword"
        ));
        assert!(is_email_pswd_valids(
            "my@email.com",
            "My cr4zy Passw0rd My cr4zy Passw0rd"
        ));
    }
}
