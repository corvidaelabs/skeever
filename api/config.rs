use crate::error::ConfigurationError;

pub struct SkeeverConfig;

impl SkeeverConfig {
    /// Get the event stream name
    pub fn get_event_stream_name() -> Result<String, ConfigurationError> {
        std::env::var("EVENT_STREAM_NAME").map_err(|e| {
            tracing::error!("Error getting event stream name {}", e);
            ConfigurationError::MissingConfiguration("EVENT_STREAM_NAME".to_string())
        })
    }

    /// Get the event stream prefix
    pub fn get_event_stream_prefix() -> Result<String, ConfigurationError> {
        std::env::var("EVENT_STREAM_PREFIX").map_err(|e| {
            tracing::error!("Error getting event stream prefix {}", e);
            ConfigurationError::MissingConfiguration("EVENT_STREAM_PREFIX".to_string())
        })
    }

    /// Get the NATS URL
    pub fn get_nats_url() -> String {
        std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string())
    }
}
