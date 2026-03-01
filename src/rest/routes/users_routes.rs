use axum::{
	routing::post,
	Router,
};

use crate::app::state::AppState;
use crate::rest::handlers::users_handlers::login_handler;

pub fn routes() -> Router<AppState> {
	Router::new().route("/user/login", post(login_handler))
}
