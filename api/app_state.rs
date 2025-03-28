use crate::{
    config::SkeeverConfig, error::SkeeverError, events::stream::EventStream,
    nats::create_nats_client, skeever::Squeak,
};
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub struct AppState {
    pub event_sender: broadcast::Sender<Squeak>,
}

impl AppState {
    pub async fn init() -> Result<Self, SkeeverError> {
        let (event_sender, _) = broadcast::channel(100);

        Ok(Self { event_sender })
    }

    pub async fn get_event_stream(&self) -> Result<Arc<EventStream>, SkeeverError> {
        let stream_name = SkeeverConfig::get_event_stream_name()?;
        let nats_client = create_nats_client().await?;
        let event_stream = EventStream::connect(stream_name, nats_client).await?;

        Ok(Arc::new(event_stream))
    }
}
