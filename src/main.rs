#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};
use futures::prelude::*;


mod constants;
mod response;
mod temp;
mod flux;

 

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();


    // let response1 = write_flux().await;
    // let r1 = response1.unwrap();

    // let response2 = read_flux().await;
    // let r2 = response2.unwrap();
    

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(temp::list)
            .service(temp::get)
            .service(temp::create)
            .service(temp::delete)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}