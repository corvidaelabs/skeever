pub mod message;
pub mod stream;
pub use stream::EventStream;

use async_nats::jetstream::{
    consumer::pull::BatchErrorKind,
    context::{CreateStreamError, DeleteStreamError, PublishError},
    stream::ConsumerError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventStreamError {
    #[error("Error creating stream")]
    StreamCreate(#[from] CreateStreamError),
    #[error("Error deleting stream")]
    StreamDelete(#[from] DeleteStreamError),
    #[error("Error creating consumer")]
    StreamConsumerCreate(#[from] ConsumerError),
    #[error("Error publishing event")]
    StreamPublish(#[from] PublishError),
    #[error("Error serializing event")]
    SerializeEvent(#[from] serde_json::Error),
    #[error("Error acknowledging event")]
    Ack(#[from] async_nats::Error),
    #[error("Error batching nats messages")]
    NatsBatch(#[from] async_nats::error::Error<BatchErrorKind>),
}
