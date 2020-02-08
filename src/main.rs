#[macro_use] extern crate serde;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use env_logger;
use bastion::prelude::*;

mod todo_api;
mod todo_api_web;

use todo_api_web::{
    routes::app_routes
};
use todo_api::db::helpers::create_table;

#[actix_rt::main]
async fn web_main() -> Result<(), std::io::Error> {    
    HttpServer::new(|| {
        App::new()
        .wrap(Logger::new("IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D"))
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
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    create_table();
    
    let _ = web_main();

    Ok(())
}
