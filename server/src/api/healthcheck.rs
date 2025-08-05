use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, NoContent},
};

use crate::AppState;

pub async fn starting(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    if state.lock().unwrap().ready {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::NO_CONTENT
    }
}

pub async fn alive() -> impl IntoResponse {
    NoContent
}

pub async fn ready(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    if state.lock().unwrap().ready {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
