use actix::prelude::*;
use actix_web::{AsyncResponder, HttpResponse, Responder, Path, HttpRequest, State};
use futures::future::Future;
use util::state::AppState;

pub fn index(req: HttpRequest<AppState>) -> impl Responder {
    send_msg!(req.state().db, super::Homepage {})
}

pub fn article_list(req: HttpRequest<AppState>) -> impl Responder {
    send_msg!(req.state().db, super::Homepage {})
}

pub fn content((name, state): (Path<String>, State<AppState>),) -> impl Responder {
    send_msg!(state.db, super::Homepage {})
}

pub fn push(req: HttpRequest<AppState>) -> impl Responder {
    send_msg!(req.state().db, super::Homepage {})
}
