mod admin;
mod blog;
mod notice;

use actix_web::{http::Method, App, Scope};
use util::state::AppState;

use controller::index;
use self::admin::admin_route;
use self::blog::blog_route;
use self::notice::notice_route;

type RouteScope = Scope<AppState>;

pub fn app_route(app: App<AppState>) -> App<AppState> {
    app.resource("", |r| r.method(Method::GET).with(index))
        .resource("/", |r| r.method(Method::GET).with(index))
        .scope("/admin", admin_route)
        .scope("/blog", blog_route)
        .scope("/notice", notice_route)
}
