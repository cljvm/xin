use actix_web::{Scope, http::Method, State, FromRequest};

use state::AppState;
use controller::blog;

pub fn blog_route<S: 'static>(scope: Scope<S>) -> Scope<S>
    where State<AppState>: FromRequest<S>
{
    scope
        .route("", Method::GET, blog::index)
        .route("/article", Method::GET, blog::article_list)
        .route("/article/{name}", Method::GET, blog::content)
        .route("/push", Method::GET, blog::push)
}