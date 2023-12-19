use axum::{extract::State, http::StatusCode, Form};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(State(state): State<AppState>, Form(form): Form<FormData>) -> StatusCode {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
