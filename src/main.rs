#[macro_use]
extern crate actix_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate jsonwebtoken as jwt;
extern crate mime;
extern crate r2d2;
extern crate serde;
extern crate serde_json;
extern crate toml;

#[macro_use]
pub mod macros;
pub mod controller;
pub mod dal;
pub mod error;
pub mod handler;
pub mod middleware;
pub mod routes;
pub mod util;

use actix::prelude::*;
use actix_web::{
    http::{header, StatusCode},
    middleware::{ErrorHandlers, Logger, Response},
    server, App, HttpRequest, HttpResponse, Result,
};
use util::config::Config;
use routes::app_route;
use util::state::AppState;

fn render_500<S>(_: &HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
    let mut builder = resp.into_builder();
    builder.header(header::CONTENT_TYPE, "application/json");
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
            .middleware(Logger::default())
            .middleware(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, render_500))
            .configure(app_route)
    }).bind(&server_url)
    .unwrap()
    .start();

    info!("Started http server: {}", &server_url);

    let _ = sys.run();
}
