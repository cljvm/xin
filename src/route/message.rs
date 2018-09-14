use actix_web::{http::Method, FromRequest, HttpRequest, Scope};

use controller::message;
use state::AppState;

pub fn message_route<S: 'static>(scope: Scope<S>) -> Scope<S>
where
    HttpRequest<AppState>: FromRequest<S>,
{
    scope
        .route("/", Method::GET, blog::index::home)
        .route("/article", Method::GET, blog::content::article_list)
        .resource("/article/{name}", |r| {
            r.name("article");
            r.method(Method::GET).with(blog::content::content);
        })
        .route("/push", Method::GET, blog::push_message::push_message_list)
        .route("/feed", Method::GET, blog::rss::rss)
        .route("/{name}", Method::GET, blog::content::source)
}
