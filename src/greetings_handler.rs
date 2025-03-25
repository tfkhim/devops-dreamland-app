use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::user_repository::UserRepository;

pub async fn greetings_handler(
    Path(user_id): Path<String>,
    State(user_repository): State<Arc<UserRepository>>,
) -> Result<String, GreetingError> {
    let display_name = user_repository.get_display_name_by_id(&user_id).await?;

    display_name
        .ok_or(GreetingError::UnknownUser { user_id })
        .map(|display_name| format!("Hello, {display_name}!"))
}

#[derive(Error, Debug)]
pub enum GreetingError {
    #[error("Unknown user with id '{user_id}'")]
    UnknownUser { user_id: String },
    #[error("An error occurred in a repository method")]
    RepositoryError(#[from] std::io::Error),
}

impl IntoResponse for GreetingError {
    fn into_response(self) -> Response {
        let status_message = match self {
            Self::UnknownUser { .. } => (StatusCode::NOT_FOUND, self.to_string()),
            Self::RepositoryError(..) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
        };

        status_message.into_response()
    }
}
