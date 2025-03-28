use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{config::SkeeverConfig, error::ConfigurationError};

pub struct SqueakBuilder {
    content: Option<String>,
    user_name: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Error, Debug)]
pub enum SqueakError {
    #[error("User name is required")]
    UserNameRequired,
    #[error("Content is required")]
    ContentRequired,
    #[error("Avatar URL is required")]
    AvatarUrlRequired,
}

impl SqueakBuilder {
    /// Sets the content of the squeak
    pub fn content(mut self, value: String) -> Self {
        self.content = Some(value);
        self
    }

    /// Sets the user name of the squeak
    pub fn user(mut self, name: String) -> Self {
        self.user_name = Some(name);
        self
    }

    /// Sets the avatar URL of the squeak
    pub fn avatar(mut self, url: String) -> Self {
        self.avatar_url = Some(url);
        self
    }

    /// Builds the squeak
    pub fn build(self) -> Result<Squeak, SqueakError> {
        let Some(user_name) = self.user_name else {
            return Err(SqueakError::UserNameRequired);
        };

        let Some(content) = self.content else {
            return Err(SqueakError::ContentRequired);
        };

        let Some(avatar_url) = self.avatar_url else {
            return Err(SqueakError::AvatarUrlRequired);
        };

        Ok(Squeak {
            id: ulid::Ulid::new(),
            content,
            author: User {
                name: user_name,
                avatar_url,
            },
        })
    }
}

impl Default for SqueakBuilder {
    fn default() -> Self {
        SqueakBuilder {
            content: None,
            user_name: None,
            avatar_url: None,
        }
    }
}

impl IntoFuture for SqueakBuilder {
    type Output = Result<Squeak, SqueakError>;
    type IntoFuture = futures::future::Ready<Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        futures::future::ready(self.build())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Squeak {
    pub id: ulid::Ulid,
    pub content: String,
    pub author: User,
}

impl Squeak {
    pub fn builder() -> SqueakBuilder {
        SqueakBuilder::default()
    }

    pub fn get_subject() -> Result<String, ConfigurationError> {
        let prefix = SkeeverConfig::get_event_stream_prefix()?;
        Ok(format!("{}.skeever.post", prefix))
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl From<Squeak> for Message {
    fn from(squeak: Squeak) -> Self {
        Message::Text(squeak.to_json().unwrap().into())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub name: String,
    pub avatar_url: String,
}
