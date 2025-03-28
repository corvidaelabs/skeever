use super::{EventStreamError, message::EventMessage};
use async_nats::{
    Client as NatsClient,
    jetstream::{
        self, Context as Jetstream,
        consumer::{Consumer, pull},
    },
};
use serde::Serialize;

/// Represents an event stream
pub struct EventStream {
    stream_name: String,
    jetstream: Jetstream,
}

impl EventStream {
    /// Connect to an existing event stream
    pub async fn connect(
        stream_name: String,
        nats_client: NatsClient,
    ) -> Result<Self, EventStreamError> {
        let jetstream = jetstream::new(nats_client);

        Ok(Self {
            stream_name,
            jetstream,
        })
    }

    /// Publish an event to the stream
    pub async fn publish<T>(&self, message: EventMessage<T>) -> Result<(), EventStreamError>
    where
        T: Serialize,
    {
        let data =
            serde_json::to_vec(&message.payload).map_err(EventStreamError::SerializeEvent)?;
        self.jetstream
            .publish(message.subject, data.into())
            .await
            .map_err(EventStreamError::StreamPublish)?;

        Ok(())
    }

    /// Creates a new consumer
    pub async fn create_consumer(
        &self,
        name: Option<String>,
        filter: String,
        deliver_policy: Option<jetstream::consumer::DeliverPolicy>,
    ) -> Result<Consumer<pull::Config>, EventStreamError> {
        let config = jetstream::consumer::pull::Config {
            durable_name: name,
            filter_subject: filter,
            max_deliver: 3,
            // Add delivery policy for historical messages
            deliver_policy: deliver_policy.unwrap_or(jetstream::consumer::DeliverPolicy::New),
            ..Default::default()
        };

        self.jetstream
            .create_consumer_on_stream(config, self.stream_name.to_string())
            .await
            .map_err(EventStreamError::StreamConsumerCreate)
    }
}
