use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list(
        &self,
        order_id: &str,
        request: &OrdersEventsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOrderEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/orders/{}/events", order_id),
                None,
                QueryBuilder::new()
                    .serialize("endingBefore", request.ending_before.clone())
                    .int("limit", request.limit.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .build(),
                options,
            )
            .await
    }
}
