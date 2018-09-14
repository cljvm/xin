mod admin;
mod blog;
mod message;

use actix_web::{http::Method, FromRequest, HttpRequest, Scope};

use self::admin::admin_route;
use self::blog::blog_route;
use self::message::message_route;

pub fn app_route<S: 'static>(scope: Scope<S>) -> Scope<S>
where
    HttpRequest<AppState>: FromRequest<S>,
{
    scope
        .nested("/admin", admin_route)
        .nested("/blog", blog_route)
        .nested("/message", message_route)
}
