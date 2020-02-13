#[macro_use] extern crate serde;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

use actix_web::{App, HttpServer};
use actix_web::middleware::{Logger,DefaultHeaders};
use bastion::prelude::*;
use uuid::Uuid;
use dotenv::dotenv;

mod schema;
mod todo_api;
mod todo_api_web;

use todo_api_web::{
    routes::app_routes,
    model::http::Clients,
};
use todo_api::db::helpers::create_table;

#[actix_rt::main]
async fn web_main() -> Result<(), std::io::Error> {  
    HttpServer::new(|| {
        App::new()
        .data(Clients::new())
        .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
        .wrap(Logger::new("IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D X-REQUEST-ID:%{x-request-id}o"))
        .configure(app_routes)
    })
    .workers(num_cpus::get() + 2)
    .bind("0.0.0.0:4000")
    .unwrap()
    .run()
    .await
}

#[fort::root]
async fn main(_: BastionContext) -> Result<(), ()> {
    dotenv().ok();
    create_table();
    
    let _ = web_main();

    Ok(())
}
