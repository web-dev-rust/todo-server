use crate::todo_api_web::controller::{
    pong, readiness,
    todo::{create_todo, show_all_todo},
    auth::{signup_user, login}
};
use actix_web::{web, HttpResponse};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            .service(
                web::scope("api/")
                    .route("create", web::post().to(create_todo))
                    .route("index", web::get().to(show_all_todo)),
            )
            .service(
                web::scope("auth/")
                    .route("signup", web::post().to(signup_user))
                    .route("login", web::post().to(login))
            )
            .route("ping", web::get().to(pong))
            .route("~/ready", web::get().to(readiness))
            .route("", web::get().to(|| HttpResponse::NotFound())),
    );
}
