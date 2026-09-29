pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseFulfillmentsItemShipping {
    #[serde(rename = "destinationType")]
    pub destination_type: CancelOrderResponseFulfillmentsItemShippingDestinationType,
    pub method: CancelOrderResponseFulfillmentsItemShippingMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<CancelOrderResponseFulfillmentsItemShippingOption>,
}

impl CancelOrderResponseFulfillmentsItemShipping {
    pub fn builder() -> CancelOrderResponseFulfillmentsItemShippingBuilder {
        <CancelOrderResponseFulfillmentsItemShippingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseFulfillmentsItemShippingBuilder {
    destination_type: Option<CancelOrderResponseFulfillmentsItemShippingDestinationType>,
    method: Option<CancelOrderResponseFulfillmentsItemShippingMethod>,
    option: Option<CancelOrderResponseFulfillmentsItemShippingOption>,
}

impl CancelOrderResponseFulfillmentsItemShippingBuilder {
    pub fn destination_type(
        mut self,
        value: CancelOrderResponseFulfillmentsItemShippingDestinationType,
    ) -> Self {
        self.destination_type = Some(value);
        self
    }

    pub fn method(mut self, value: CancelOrderResponseFulfillmentsItemShippingMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn option(mut self, value: CancelOrderResponseFulfillmentsItemShippingOption) -> Self {
        self.option = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponseFulfillmentsItemShipping`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination_type`](CancelOrderResponseFulfillmentsItemShippingBuilder::destination_type)
    /// - [`method`](CancelOrderResponseFulfillmentsItemShippingBuilder::method)
    pub fn build(self) -> Result<CancelOrderResponseFulfillmentsItemShipping, BuildError> {
        Ok(CancelOrderResponseFulfillmentsItemShipping {
            destination_type: self
                .destination_type
                .ok_or_else(|| BuildError::missing_field("destination_type"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            option: self.option,
        })
    }
}
