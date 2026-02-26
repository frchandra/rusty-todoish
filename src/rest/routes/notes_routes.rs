use axum::{
    Router,
    routing::{get},
};

use crate::rest::handlers::notes_handlers::{create_note_handler, delete_note_handler, get_note_handler, note_list_handler, update_note_handler};

use crate::app::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/notes", get(note_list_handler).post(create_note_handler))
        .route(
            "/notes/{id}",
            get(get_note_handler)
                .put(update_note_handler)
                .delete(delete_note_handler),
        )
}
