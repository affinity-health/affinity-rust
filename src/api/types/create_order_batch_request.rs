pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateOrderBatchRequest {
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber: Option<CreateOrderBatchRequestPrescriber>,
    #[serde(default)]
    pub orders: Vec<CreateOrderBatchRequestOrdersItem>,
}

impl CreateOrderBatchRequest {
    pub fn builder() -> CreateOrderBatchRequestBuilder {
        <CreateOrderBatchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestBuilder {
    practice_id: Option<String>,
    user_id: Option<String>,
    prescriber: Option<CreateOrderBatchRequestPrescriber>,
    orders: Option<Vec<CreateOrderBatchRequestOrdersItem>>,
}

impl CreateOrderBatchRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn prescriber(mut self, value: CreateOrderBatchRequestPrescriber) -> Self {
        self.prescriber = Some(value);
        self
    }

    pub fn orders(mut self, value: Vec<CreateOrderBatchRequestOrdersItem>) -> Self {
        self.orders = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`practice_id`](CreateOrderBatchRequestBuilder::practice_id)
    /// - [`orders`](CreateOrderBatchRequestBuilder::orders)
    pub fn build(self) -> Result<CreateOrderBatchRequest, BuildError> {
        Ok(CreateOrderBatchRequest {
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            user_id: self.user_id,
            prescriber: self.prescriber,
            orders: self
                .orders
                .ok_or_else(|| BuildError::missing_field("orders"))?,
        })
    }
}
