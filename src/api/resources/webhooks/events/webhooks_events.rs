use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EventsClient2 {
    pub http_client: HttpClient,
}

impl EventsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list(
        &self,
        request: &WebhooksEventsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhookEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/webhook-events",
                None,
                QueryBuilder::new()
                    .serialize("endingBefore", request.ending_before.clone())
                    .int("limit", request.limit.clone())
                    .serialize("status", request.status.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get(
        &self,
        event_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetWebhookEventResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/webhook-events/{}", event_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn replay(
        &self,
        event_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReplayWebhookEventResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/webhook-events/{}/replay", event_id),
                None,
                None,
                options,
            )
            .await
    }
}
