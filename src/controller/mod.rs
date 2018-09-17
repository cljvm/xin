pub mod admin;
pub mod blog;
pub mod notice;

use actix::prelude::*;
use actix_web::{AsyncResponder, HttpResponse, Responder, State};
use futures::future::Future;
use state::AppState;
use actix_web::Error;

#[derive(Message)]
#[rtype(result = "Result<String, Error>")]
pub struct Homepage {}

pub fn index(state: State<AppState>) -> impl Responder {
    send_msg!(state.db, Homepage {})
    // state
    //     .db
    //     .send(Homepage)
    //     .from_err()
    //     .and_then(|res| match res {
    //         Ok(s) => Ok(HttpResponse::Ok().json(s)),
    //         Err(_) => Ok(HttpResponse::InternalServerError().into()),
    //     })
    //     .responder()
}
