pub mod admin;
pub mod blog;
pub mod notice;

use actix::prelude::*;
use actix_web::{AsyncResponder, HttpResponse, Responder, HttpRequest, State, Json};
use futures::future::Future;
use util::state::AppState;
use actix_web::Error;

#[derive(Message)]
#[rtype(result = "Result<String, Error>")]
pub struct Homepage {}

#[derive(Message, Serialize)]
#[rtype(result = "Result<String, Error>")]
pub struct SignIn {
    identifier: String,
    credential: String,
}

#[derive(Message, Serialize)]
#[rtype(result = "Result<String, Error>")]
pub struct CreateAccount {
    name: String,
    identifier: String,
    credential: String,
}

pub fn index(state: State<AppState>,) -> impl Responder {
    send_msg!(state.db, Homepage {})
}

pub fn sign_in((state, sign_in): (State<AppState>, Json<SignIn>)) -> impl Responder {
    send_msg!(state.db, Homepage {})
}

pub fn sign_out(req: HttpRequest<AppState>) -> impl Responder {
    send_msg!(req.state().db, Homepage {})
}

pub fn register((state, account): (State<AppState>, Json<CreateAccount>)) -> impl Responder {
    send_msg!(state.db, Homepage {})
}