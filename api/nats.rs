use crate::{config::SkeeverConfig, error::ConfigurationError};

/// Create a NATS client using the provided URL
pub async fn create_nats_client() -> Result<async_nats::Client, ConfigurationError> {
    let url = SkeeverConfig::get_nats_url();
    async_nats::connect(url)
        .await
        .map_err(ConfigurationError::NatsConnect)
}
