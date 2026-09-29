pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOrderBatchResponse {
    pub object: CreateOrderBatchResponseObject,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub livemode: bool,
    #[serde(default)]
    pub orders: Vec<CreateOrderBatchResponseOrdersItem>,
}

impl CreateOrderBatchResponse {
    pub fn builder() -> CreateOrderBatchResponseBuilder {
        <CreateOrderBatchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchResponseBuilder {
    object: Option<CreateOrderBatchResponseObject>,
    practice_id: Option<String>,
    user_id: Option<String>,
    livemode: Option<bool>,
    orders: Option<Vec<CreateOrderBatchResponseOrdersItem>>,
}

impl CreateOrderBatchResponseBuilder {
    pub fn object(mut self, value: CreateOrderBatchResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn orders(mut self, value: Vec<CreateOrderBatchResponseOrdersItem>) -> Self {
        self.orders = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object`](CreateOrderBatchResponseBuilder::object)
    /// - [`practice_id`](CreateOrderBatchResponseBuilder::practice_id)
    /// - [`livemode`](CreateOrderBatchResponseBuilder::livemode)
    /// - [`orders`](CreateOrderBatchResponseBuilder::orders)
    pub fn build(self) -> Result<CreateOrderBatchResponse, BuildError> {
        Ok(CreateOrderBatchResponse {
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            user_id: self.user_id,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            orders: self
                .orders
                .ok_or_else(|| BuildError::missing_field("orders"))?,
        })
    }
}
