use async_nats::ConnectError;
use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::{events::EventStreamError, websockets::WebsocketError};

#[derive(Error, Debug)]
pub enum SkeeverError {
    #[error("Configuration error")]
    Configuration(#[from] ConfigurationError),
    #[error("Event Stream error")]
    EventStream(#[from] EventStreamError),
    #[error("Websocket error")]
    Websocket(#[from] WebsocketError),
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Missing configuration: {0}")]
    MissingConfiguration(String),
    #[error("Invalid NATS configuration")]
    NatsConnect(#[from] ConnectError),
}

impl IntoResponse for SkeeverError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            SkeeverError::Configuration(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
            SkeeverError::EventStream(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Event Stream error")
            }
            SkeeverError::Websocket(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Websocket error"),
        };
        (status, error_message).into_response()
    }
}
