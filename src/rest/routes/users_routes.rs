use axum::{
	routing::post,
	Router,
};

use crate::app::state::AppState;
use crate::rest::handlers::auth_handlers::*;

pub fn routes() -> Router<AppState> {
	Router::new()
        .route("/user/login", post(login_handler))
		.route("/user/logout", post(logout_handler))
        .route("/revoke-all", post(revoke_all_handler))
}
