pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetOrderResponseFulfillmentsItemShipping {
    #[serde(rename = "destinationType")]
    pub destination_type: GetOrderResponseFulfillmentsItemShippingDestinationType,
    pub method: GetOrderResponseFulfillmentsItemShippingMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<GetOrderResponseFulfillmentsItemShippingOption>,
}

impl GetOrderResponseFulfillmentsItemShipping {
    pub fn builder() -> GetOrderResponseFulfillmentsItemShippingBuilder {
        <GetOrderResponseFulfillmentsItemShippingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponseFulfillmentsItemShippingBuilder {
    destination_type: Option<GetOrderResponseFulfillmentsItemShippingDestinationType>,
    method: Option<GetOrderResponseFulfillmentsItemShippingMethod>,
    option: Option<GetOrderResponseFulfillmentsItemShippingOption>,
}

impl GetOrderResponseFulfillmentsItemShippingBuilder {
    pub fn destination_type(
        mut self,
        value: GetOrderResponseFulfillmentsItemShippingDestinationType,
    ) -> Self {
        self.destination_type = Some(value);
        self
    }

    pub fn method(mut self, value: GetOrderResponseFulfillmentsItemShippingMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn option(mut self, value: GetOrderResponseFulfillmentsItemShippingOption) -> Self {
        self.option = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponseFulfillmentsItemShipping`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination_type`](GetOrderResponseFulfillmentsItemShippingBuilder::destination_type)
    /// - [`method`](GetOrderResponseFulfillmentsItemShippingBuilder::method)
    pub fn build(self) -> Result<GetOrderResponseFulfillmentsItemShipping, BuildError> {
        Ok(GetOrderResponseFulfillmentsItemShipping {
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
