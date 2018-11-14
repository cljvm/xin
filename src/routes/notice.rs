use actix_web::http::Method;
use controller::notice;

pub fn notice_route(scope: super::RouteScope) -> super::RouteScope {
    scope
        .route("", Method::GET, notice::index)
        .route("/{name}", Method::GET, notice::content)
}
