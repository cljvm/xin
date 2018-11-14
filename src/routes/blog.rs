use actix_web::http::Method;
use controller::blog;

pub fn blog_route(scope: super::RouteScope) -> super::RouteScope {
    scope
        .route("", Method::GET, blog::index)
        .route("/article", Method::GET, blog::article_list)
        .resource("/article/{name}", |r| {
            r.method(Method::GET).with(blog::content)
        }).route("/push", Method::GET, blog::push)
}
