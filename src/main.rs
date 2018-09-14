#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate log;

extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate env_logger;
extern crate mime;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
extern crate serde_json;

mod db;
mod state;
mod route;
mod error;
mod controller;
mod handler;
mod middleware;
mod model;
mod schema;

use actix::prelude::System;
use actix_web::middleware::{ErrorHandlers, Logger, Response};
use actix_web::{server, http, App, HttpRequest, HttpResponse, Result};
use route::app_route;

fn render_500<S>(_: &HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
    let mut builder = resp.into_builder();
    builder.header(http::header::CONTENT_TYPE, "application/json");
    Ok(Response::Done(builder.into()))
}

fn main() {
    env_logger::init();
    let sys = System::new("lemon");

    // Start http server
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            // enable logger
            .middleware(Logger::default())
            .middleware(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500))
            .scope("", app_route)
            .resource("/{name}", |r| r.method(http::Method::GET).with(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
