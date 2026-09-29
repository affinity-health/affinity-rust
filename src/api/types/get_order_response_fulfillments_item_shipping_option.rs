pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetOrderResponseFulfillmentsItemShippingOption {
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    pub currency: GetOrderResponseFulfillmentsItemShippingOptionCurrency,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "serviceLevel")]
    #[serde(default)]
    pub service_level: String,
    pub temperature: GetOrderResponseFulfillmentsItemShippingOptionTemperature,
}

impl GetOrderResponseFulfillmentsItemShippingOption {
    pub fn builder() -> GetOrderResponseFulfillmentsItemShippingOptionBuilder {
        <GetOrderResponseFulfillmentsItemShippingOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponseFulfillmentsItemShippingOptionBuilder {
    amount_cents: Option<i64>,
    currency: Option<GetOrderResponseFulfillmentsItemShippingOptionCurrency>,
    label: Option<String>,
    service_level: Option<String>,
    temperature: Option<GetOrderResponseFulfillmentsItemShippingOptionTemperature>,
}

impl GetOrderResponseFulfillmentsItemShippingOptionBuilder {
    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn currency(
        mut self,
        value: GetOrderResponseFulfillmentsItemShippingOptionCurrency,
    ) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn service_level(mut self, value: impl Into<String>) -> Self {
        self.service_level = Some(value.into());
        self
    }

    pub fn temperature(
        mut self,
        value: GetOrderResponseFulfillmentsItemShippingOptionTemperature,
    ) -> Self {
        self.temperature = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponseFulfillmentsItemShippingOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_cents`](GetOrderResponseFulfillmentsItemShippingOptionBuilder::amount_cents)
    /// - [`currency`](GetOrderResponseFulfillmentsItemShippingOptionBuilder::currency)
    /// - [`label`](GetOrderResponseFulfillmentsItemShippingOptionBuilder::label)
    /// - [`service_level`](GetOrderResponseFulfillmentsItemShippingOptionBuilder::service_level)
    /// - [`temperature`](GetOrderResponseFulfillmentsItemShippingOptionBuilder::temperature)
    pub fn build(self) -> Result<GetOrderResponseFulfillmentsItemShippingOption, BuildError> {
        Ok(GetOrderResponseFulfillmentsItemShippingOption {
            amount_cents: self
                .amount_cents
                .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            service_level: self
                .service_level
                .ok_or_else(|| BuildError::missing_field("service_level"))?,
            temperature: self
                .temperature
                .ok_or_else(|| BuildError::missing_field("temperature"))?,
        })
    }
}
