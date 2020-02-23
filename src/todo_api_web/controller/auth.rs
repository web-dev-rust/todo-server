use actix_web::{HttpResponse, HttpRequest, web, Responder};
use log::{error};
use crate::{
    todo_api_web::model::{
        http::Clients,
        auth::{SignUp, Auth}
    }
};
use crate::todo_api::{
    core::{
        generate_jwt, decode_jwt, validate_jwt_date, validate_jwt_info, is_email_pswd_valids,
        model::JwtValue,
    }
};

pub async fn signup_user(state: web::Data<Clients>, info: web::Json<SignUp>) -> impl Responder {
    let signup = info.into_inner();
    if !is_email_pswd_valids(&signup.email, &signup.password) {
        return HttpResponse::BadRequest();
    }

    let resp = state.postgres
        .send(signup)
        .await;

    match resp {
        Ok(_) => HttpResponse::Created(),
        Err(e) => {
            error!("{:?}",e);
            HttpResponse::InternalServerError()
        },
    }
}

pub async fn login(state: web::Data<Clients>, info: web::Json<Auth>) -> impl Responder {
    let login_user = info.clone();
    if !is_email_pswd_valids(&login_user.email, &login_user.password.clone().unwrap()) {
        return HttpResponse::BadRequest().finish();
    }

    let resp = state.postgres
        .send(login_user)
        .await;

    match resp {
        Err(e)  => {
            error!("{:?}",e);
            HttpResponse::NoContent().finish()
        },
        Ok(user) => {
            let usr = user.unwrap();
            match usr.verify(info.clone().password.unwrap()) {
                Ok(true) => generate_jwt(usr, state).await,
                Ok(false) => HttpResponse::NoContent().finish(),
                Err(_) => HttpResponse::NoContent().finish()
            }
        }
    }
}

pub async fn logout(req: HttpRequest, state: web::Data<Clients>, info: web::Json<Auth>) -> impl Responder {
    use regex::Regex;

    let jwt = req.headers().get("x-auth");
    let logout_user = info.clone();
    let email_regex = Regex::new("\\w{1,}@\\w{2,}.[a-z]{2,3}(.[a-z]{2,3})?$").unwrap();
    
    if !email_regex.is_match(&logout_user.email) {
        return HttpResponse::BadRequest().finish();
    }

    let resp = state.postgres
        .send(logout_user.clone());

    match jwt {
        None => return HttpResponse::BadRequest().finish(),
        Some(jwt) => {
            let jwt_value : JwtValue = serde_json::from_value(decode_jwt(jwt.to_str().unwrap())).expect("failed to parse JWT Value");
            match validate_jwt_date(jwt_value.expires_at) {
                false => 
                    HttpResponse::Unauthorized().finish(),
                true => {
                    validate_jwt_info(jwt_value.email, logout_user.email, resp.await.expect("Failed to read contact info"))
                }
            }
        }
    }
}