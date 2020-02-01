use crate::todo_api_web::controller::{pong, readiness, todo::create_todo};
use actix_web::{web, HttpResponse};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            .service(web::scope("api/").route("create", web::post().to(create_todo)))
            .route("ping", web::get().to(pong))
            .route("~/ready", web::get().to(readiness))
            .route("", web::get().to(|| HttpResponse::NotFound())),
    );
}
