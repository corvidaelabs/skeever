use serde::{Deserialize, Serialize};

use crate::skeever::Squeak;

#[derive(Deserialize, Serialize)]
pub struct EventMessage<T>
where
    T: Serialize,
{
    pub subject: String,
    pub payload: T,
}

impl From<Squeak> for EventMessage<Squeak> {
    fn from(squeak: Squeak) -> Self {
        let subject = Squeak::get_subject().expect("Failed to get subject for squeak");
        EventMessage {
            subject,
            payload: squeak,
        }
    }
}
