pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CatalogShippingOptionsListQueryRequest {
    #[serde(rename = "destinationState")]
    #[serde(default)]
    pub destination_state: String,
    #[serde(rename = "destinationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<ListShippingOptionsRequestDestinationType>,
}

impl CatalogShippingOptionsListQueryRequest {
    pub fn builder() -> CatalogShippingOptionsListQueryRequestBuilder {
        <CatalogShippingOptionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CatalogShippingOptionsListQueryRequestBuilder {
    destination_state: Option<String>,
    destination_type: Option<ListShippingOptionsRequestDestinationType>,
}

impl CatalogShippingOptionsListQueryRequestBuilder {
    pub fn destination_state(mut self, value: impl Into<String>) -> Self {
        self.destination_state = Some(value.into());
        self
    }

    pub fn destination_type(mut self, value: ListShippingOptionsRequestDestinationType) -> Self {
        self.destination_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CatalogShippingOptionsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination_state`](CatalogShippingOptionsListQueryRequestBuilder::destination_state)
    pub fn build(self) -> Result<CatalogShippingOptionsListQueryRequest, BuildError> {
        Ok(CatalogShippingOptionsListQueryRequest {
            destination_state: self
                .destination_state
                .ok_or_else(|| BuildError::missing_field("destination_state"))?,
            destination_type: self.destination_type,
        })
    }
}
