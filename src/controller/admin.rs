use actix::prelude::*;
use actix_web::{AsyncResponder, HttpResponse, Responder, State};
use futures::future::Future;
use state::AppState;

pub fn index(state: State<AppState>) -> impl Responder {
    send_msg!(state.db, super::Homepage {})
}
