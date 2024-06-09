use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::user_repository::UserRepository;

pub async fn greetings_handler<'de>(
    Path(user_id): Path<String>,
    State(user_repository): State<Arc<UserRepository>>,
) -> Result<String, GreetingError> {
    user_repository
        .get_display_name_by_id(&user_id)
        .await
        .ok_or(GreetingError::UnknownUser { user_id })
        .map(|display_name| format!("Hello, {display_name}!"))
}

#[derive(Error, Debug)]
pub enum GreetingError {
    #[error("Unknown user with id '{user_id}'")]
    UnknownUser { user_id: String },
}

impl IntoResponse for GreetingError {
    fn into_response(self) -> Response {
        let message = self.to_string();

        let status = match self {
            Self::UnknownUser { .. } => StatusCode::NOT_FOUND,
        };

        (status, message).into_response()
    }
}
