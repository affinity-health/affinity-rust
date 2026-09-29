use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod endpoints;
pub use endpoints::EndpointsClient;
pub mod events;
pub use events::EventsClient2;
pub mod grants;
pub use grants::GrantsClient;
pub struct WebhooksClient {
    pub http_client: HttpClient,
    pub endpoints: EndpointsClient,
    pub events: EventsClient2,
    pub grants: GrantsClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            endpoints: EndpointsClient::new(config.clone())?,
            events: EventsClient2::new(config.clone())?,
            grants: GrantsClient::new(config.clone())?,
        })
    }
}
