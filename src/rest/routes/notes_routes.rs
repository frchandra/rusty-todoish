use axum::{Router, routing::get};

use crate::rest::handlers::notes_handlers::{get_note_handler, note_list_handler};

use crate::app::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/notes", get(note_list_handler))
        .route("/notes/{id}", get(get_note_handler))
}
