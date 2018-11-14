use actix_web::http::Method;
use controller::admin;

pub fn admin_route(scope: super::RouteScope) -> super::RouteScope {
    scope.route("", Method::GET, admin::index)
}
