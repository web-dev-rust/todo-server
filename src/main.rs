use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn pong() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

async fn readiness() -> impl Responder {
    let process = std::process::Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output();

    match process {
        Ok(_) => HttpResponse::Accepted(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/")
                .route("ping", web::get().to(pong))
                .route("~/ready", web::get().to(readiness))
                .route("", web::get().to(|| HttpResponse::NotFound())),
        )
    })
    .workers(6)
    .bind("127.0.0.1:4000")
    .unwrap()
    .run()
    .await
}
