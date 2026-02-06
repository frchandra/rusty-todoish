use axum::{
    Router,
    routing::get
};

use crate::{
    rest::handlers::notes_handlers::{
        note_list_handler
    }
};

use crate::app::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/notes", get(note_list_handler))
}
