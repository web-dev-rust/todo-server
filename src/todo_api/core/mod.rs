use actix_web::{HttpResponse,  web};
use log::error;
use crate::todo_api::{
    model::auth::User,
    core::model::UpdateDate,
};
use crate::todo_api_web::model::http::Clients;

pub mod model;

pub async fn generate_jwt(user: User, state: web::Data<Clients>) -> HttpResponse {
    let utc = crate::todo_api::db::helpers::one_day_from_now().naive_utc();

    let update_date = UpdateDate {
        email: user.email.clone(),
        expires_at: utc,
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
    let header = json!({ "alg": alg.name(), "date":  Utc::now().to_string()});
    let claims = json!({ "id": user.clone().get_id(), "email": user.email, "expires_at": update_date.expires_at });
    encode(&header, &claims, &alg).unwrap()
}
