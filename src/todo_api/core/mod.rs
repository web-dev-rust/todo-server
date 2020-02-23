use actix_web::{HttpResponse,  web};
use log::error;
use serde_json::value::Value;

use crate::todo_api::{
    db::error::DbError,
    model::auth::User,
    core::model::{UpdateDate}
};
use crate::todo_api_web::model::http::Clients;

pub mod model;

pub async fn generate_jwt(user: User, state: web::Data<Clients>) -> HttpResponse {
    let utc = crate::todo_api::db::helpers::one_day_from_now().naive_utc();

    let update_date = UpdateDate {
        email: user.email.clone(),
        expires_at: utc,
        is_active: true,
    };

    let resp = state.postgres
        .send(update_date.clone())
        .await;

    match resp {
        Ok(_) => { 
            let token_jwt = create_token(user, update_date);
            let jwt = crate::todo_api::core::model::Jwt::new(token_jwt);
            HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&jwt).unwrap())
        },
        Err(e) => {
            error!("{:?}",e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub fn create_token(user: User, update_date: UpdateDate) -> String {
    use serde_json::json;
    use jsonwebtokens::{Algorithm, AlgorithmID, encode};
    use chrono::Utc;

    let alg = Algorithm::new_hmac(AlgorithmID::HS256, "secret").unwrap();
    let header = json!({ "alg": alg.name(), "typ": "jwt", "date":  Utc::now().to_string()});
    let payload = json!({ "id": user.clone().get_id(), "email": user.email, "expires_at": update_date.expires_at });
    encode(&header, &payload, &alg).unwrap()
}

pub fn decode_jwt(jwt: &str) -> Value {
    use jsonwebtokens::raw::{TokenSlices, split_token, decode_json_token_slice};

    let TokenSlices {claims, .. } = split_token(jwt).unwrap();
    let claims = decode_json_token_slice(claims).expect("Failed to decode token");
    claims
}

pub fn validate_jwt_date(jwt_expires: chrono::NaiveDateTime) -> bool {
    chrono::Utc::now().naive_utc() <= jwt_expires
}

pub fn validate_jwt_info(jwt_email: String, req_email: String, user: Result<User, DbError>) -> HttpResponse {
    match user {
        Err(_) => HttpResponse::Unauthorized().finish(),
        Ok(u) => {
            if u.email == jwt_email && jwt_email == req_email {
                HttpResponse::Accepted().finish()
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
    use chrono::{DateTime, Utc};
    use actix_web::http::StatusCode;
    use crate::todo_api::model::auth::User;

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

    #[test]
    fn all_args_are_equal_is_accepted() {
        let user = User::from("my@email.com".to_string(), "pass".to_string());
        let email = "my@email.com".to_string();

        assert_eq!(validate_jwt_info(email.clone(), email, Ok(user)).status(), StatusCode::ACCEPTED);
    }

    #[test]
    fn all_args_are_not_equal_is_unauth() {
        let user = User::from("not_my@email.com".to_string(), "pass".to_string());
        let email = "my@email.com".to_string();

        assert_eq!(validate_jwt_info(email.clone(), email, Ok(user)).status(), StatusCode::UNAUTHORIZED);
    }
}

#[cfg(test)]
mod valid_email_pswd {
    use super::is_email_pswd_valids;

    #[test]
    fn valid_email_and_pswd() {
        assert!(is_email_pswd_valids("my@email.com", "My cr4zy P@ssw0rd My cr4zy P@ssw0rd"));
    }

    #[test]
    fn invalid_emails() {
        assert!(!is_email_pswd_valids("my_email.com", "My cr4zy P@ssw0rd My cr4zy P@ssw0rd"));
        assert!(!is_email_pswd_valids("my@email.com.br.us", "My cr4zy P@ssw0rd My cr4zy P@ssw0rd"));
    }

    #[test]
    fn invalid_passwords() {
        assert!(!is_email_pswd_valids("my@email.com.br", "My cr4zy P@ssw0rd"));
        assert!(is_email_pswd_valids("my@email.com", "my cr4zy p@ssw0rd my cr4zy p@ssw0rd"));
        assert!(is_email_pswd_valids("my@email.com", "My crazy P@ssword My crazy P@ssword"));
        assert!(is_email_pswd_valids("my@email.com", "My cr4zy Passw0rd My cr4zy Passw0rd"));
    }
}