pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListOrdersResponseDataItemFulfillmentsItemShipping {
    #[serde(rename = "destinationType")]
    pub destination_type: ListOrdersResponseDataItemFulfillmentsItemShippingDestinationType,
    pub method: ListOrdersResponseDataItemFulfillmentsItemShippingMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<ListOrdersResponseDataItemFulfillmentsItemShippingOption>,
}

impl ListOrdersResponseDataItemFulfillmentsItemShipping {
    pub fn builder() -> ListOrdersResponseDataItemFulfillmentsItemShippingBuilder {
        <ListOrdersResponseDataItemFulfillmentsItemShippingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemFulfillmentsItemShippingBuilder {
    destination_type: Option<ListOrdersResponseDataItemFulfillmentsItemShippingDestinationType>,
    method: Option<ListOrdersResponseDataItemFulfillmentsItemShippingMethod>,
    option: Option<ListOrdersResponseDataItemFulfillmentsItemShippingOption>,
}

impl ListOrdersResponseDataItemFulfillmentsItemShippingBuilder {
    pub fn destination_type(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemShippingDestinationType,
    ) -> Self {
        self.destination_type = Some(value);
        self
    }

    pub fn method(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemShippingMethod,
    ) -> Self {
        self.method = Some(value);
        self
    }

    pub fn option(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemShippingOption,
    ) -> Self {
        self.option = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemFulfillmentsItemShipping`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination_type`](ListOrdersResponseDataItemFulfillmentsItemShippingBuilder::destination_type)
    /// - [`method`](ListOrdersResponseDataItemFulfillmentsItemShippingBuilder::method)
    pub fn build(self) -> Result<ListOrdersResponseDataItemFulfillmentsItemShipping, BuildError> {
        Ok(ListOrdersResponseDataItemFulfillmentsItemShipping {
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
