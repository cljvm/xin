#[macro_use]
extern crate actix_derive;
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
extern crate futures;
extern crate chrono;
extern crate env_logger;
extern crate mime;
extern crate r2d2;
extern crate serde;
extern crate serde_json;
extern crate toml;

#[macro_use]
pub mod macros;
pub mod auth;
pub mod config;
pub mod controller;
pub mod db;
pub mod error;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod route;
pub mod schema;
pub mod state;

use actix::prelude::System;
use actix_web::middleware::{ErrorHandlers, Logger, Response};
use actix_web::{http, server, App, HttpRequest, HttpResponse, Result};
use config::Config;
use route::app_route;
use state::AppState;

fn render_500<S>(_: &HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
    let mut builder = resp.into_builder();
    builder.header(http::header::CONTENT_TYPE, "application/json");
    Ok(Response::Done(builder.into()))
}

fn main() {
    env_logger::init();

    let config = Config::from_file("config.toml").unwrap();
    let sys = System::new("xin");

    let state = AppState::new(&config);

    let server_url = config.server.to_string();

    // Start http server
    server::new(move || {
        App::with_state(state.clone())
            // enable logger
            .middleware(Logger::default())
            .middleware(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500))
            .scope("", app_route)
    }).bind(&server_url)
        .unwrap()
        .start();

    info!("Started http server: {}", &server_url);

    let _ = sys.run();
}
