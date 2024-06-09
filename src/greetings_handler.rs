use std::sync::Arc;

use axum::extract::{Path, State};

use crate::user_repository::UserRepository;

pub async fn greetings_handler<'de>(
    Path(name): Path<String>,
    State(user_repository): State<Arc<UserRepository>>,
) -> String {
    let display_name = user_repository.get_display_name_for_username(&name).await;

    format!("Hello, {}!", display_name.unwrap_or("World".to_owned()))
}
