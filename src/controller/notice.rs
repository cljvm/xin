use actix::prelude::*;
use actix_web::{AsyncResponder, HttpResponse, Responder, HttpRequest};
use futures::future::Future;
use util::state::AppState;

pub fn index(req: HttpRequest<AppState>) -> impl Responder {
    send_msg!(req.state().db, super::Homepage {})
}

pub fn content(req: HttpRequest<AppState>) -> impl Responder {
    send_msg!(req.state().db, super::Homepage {})
}
