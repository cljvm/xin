mod admin;
mod blog;
mod notice;

use actix_web::{http::Method, FromRequest, State, Scope};
use state::AppState;
use controller::index;

use self::admin::admin_route;
use self::blog::blog_route;
use self::notice::notice_route;

pub fn app_route<S: 'static>(scope: Scope<S>) -> Scope<S>
where
    State<AppState>: FromRequest<S>,
{
    scope
        .route("", Method::GET, index)
        .route("/", Method::GET, index)
        .nested("/admin", admin_route)
        .nested("/blog", blog_route)
        .nested("/notice", notice_route)
}
