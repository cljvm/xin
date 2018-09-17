use actix_web::{Scope, http::Method, State, FromRequest};

use state::AppState;
use controller::admin;

pub fn admin_route<S: 'static>(scope: Scope<S>) -> Scope<S>
    where State<AppState>: FromRequest<S>
{
    scope
        .route("", Method::GET, admin::index)
}