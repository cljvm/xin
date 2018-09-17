use actix_web::{HttpRequest, HttpResponse, Result};
use actix_web::middleware::{Middleware, Started};
use actix_web::middleware::session::RequestSession;

use auth::PrivateClaims;
use error::ErrorMessage;

pub struct Authenticate;

impl<S> Middleware<S> for Authenticate {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        if let Some(_) = req.session().get::<PrivateClaims>("claims")? {
            Ok(Started::Done)
        } else {
            Ok(Started::Response(HttpResponse::Unauthorized().json(ErrorMessage {
                code: 401,
                msg: "Invalid authentication token.".to_owned(),
                detail: None,
            })))
        }
    }
}