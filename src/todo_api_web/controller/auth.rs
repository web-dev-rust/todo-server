use actix_web::{HttpResponse, web, Responder};
use log::{error};
use crate::{
    todo_api_web::model::{
        http::Clients,
        auth::SignUp,
    }
};

pub async fn signup_user(state: web::Data<Clients>, info: web::Json<SignUp>) -> impl Responder {
    use regex::Regex;

    let email_regex = Regex::new("\\w{1,}@\\w{2,}.[a-z]{2,3}(.[a-z]{2,3})?").unwrap();
    let pswd_regex = Regex::new("[[a-z]+[A-Z]+[0-9]+(\\s@!=_#&~\\[\\]\\{\\}\\?\\/)]{32,64}").unwrap();
    
    let signup = info.into_inner();
    if !email_regex.is_match(&signup.email) || !pswd_regex.is_match(&signup.password) {
        return HttpResponse::BadRequest();
    }

    let resp = state.postgres.clone().unwrap()
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
