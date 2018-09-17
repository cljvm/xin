use actix_web::{http::Method, FromRequest, State, Scope};

use controller::notice;
use state::AppState;

pub fn notice_route<S: 'static>(scope: Scope<S>) -> Scope<S>
where
    State<AppState>: FromRequest<S>,
{
    scope
        .route("", Method::GET, notice::index)
        .route("/{name}", Method::GET, notice::content)
}
