pub use crate::prelude::*;

/// Query parameters for listShippingOptions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListShippingOptionsQueryRequest {
    #[serde(rename = "destinationState")]
    #[serde(default)]
    pub destination_state: String,
    #[serde(rename = "destinationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<ListShippingOptionsRequestDestinationType>,
}

impl ListShippingOptionsQueryRequest {
    pub fn builder() -> ListShippingOptionsQueryRequestBuilder {
        <ListShippingOptionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListShippingOptionsQueryRequestBuilder {
    destination_state: Option<String>,
    destination_type: Option<ListShippingOptionsRequestDestinationType>,
}

impl ListShippingOptionsQueryRequestBuilder {
    pub fn destination_state(mut self, value: impl Into<String>) -> Self {
        self.destination_state = Some(value.into());
        self
    }

    pub fn destination_type(mut self, value: ListShippingOptionsRequestDestinationType) -> Self {
        self.destination_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListShippingOptionsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination_state`](ListShippingOptionsQueryRequestBuilder::destination_state)
    pub fn build(self) -> Result<ListShippingOptionsQueryRequest, BuildError> {
        Ok(ListShippingOptionsQueryRequest {
            destination_state: self
                .destination_state
                .ok_or_else(|| BuildError::missing_field("destination_state"))?,
            destination_type: self.destination_type,
        })
    }
}
