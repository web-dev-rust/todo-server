use actix_web::{HttpResponse, web, Responder};
use log::{error};
use crate::{
    todo_api_web::model::{
        http::Clients,
        auth::{SignUp, Login}
    }
};
use crate::todo_api::{
    core::generate_jwt,
};

pub async fn signup_user(state: web::Data<Clients>, info: web::Json<SignUp>) -> impl Responder {
    use regex::Regex;

    let email_regex = Regex::new("\\w{1,}@\\w{2,}.[a-z]{2,3}(.[a-z]{2,3})?").unwrap();
    let pswd_regex = Regex::new("[[a-z]+[A-Z]+[0-9]+(\\s@!=_#&~\\[\\]\\{\\}\\?)]{32,64}").unwrap();
    
    let signup = info.into_inner();
    if !email_regex.is_match(&signup.email) || !pswd_regex.is_match(&signup.password) {
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

pub async fn login(state: web::Data<Clients>, info: web::Json<Login>) -> impl Responder {
    use regex::Regex;

    let email_regex = Regex::new("\\w{1,}@\\w{2,}.[a-z]{2,3}(.[a-z]{2,3})?").unwrap();
    let pswd_regex = Regex::new("[[a-z]+[A-Z]+[0-9]+(\\s@!=_#&~\\[\\]\\{\\}\\?)]{32,64}").unwrap();
    
    let login_user = info.clone();
    if !email_regex.is_match(&login_user.email) || !pswd_regex.is_match(&login_user.password) {
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
            match usr.verify(info.clone().password) {
                Ok(true) => generate_jwt(usr, state).await,
                Ok(false) => HttpResponse::NoContent().finish(),
                Err(_) => HttpResponse::NoContent().finish()
            }
        }
    }
}

