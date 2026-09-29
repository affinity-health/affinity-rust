pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListShippingOptionsResponseItem {
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    pub currency: ListShippingOptionsResponseItemCurrency,
    #[serde(rename = "estimatedDaysMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_days_max: Option<i64>,
    #[serde(rename = "estimatedDaysMin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_days_min: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "serviceLevel")]
    #[serde(default)]
    pub service_level: String,
    pub temperature: ListShippingOptionsResponseItemTemperature,
}

impl ListShippingOptionsResponseItem {
    pub fn builder() -> ListShippingOptionsResponseItemBuilder {
        <ListShippingOptionsResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListShippingOptionsResponseItemBuilder {
    amount_cents: Option<i64>,
    carrier: Option<String>,
    currency: Option<ListShippingOptionsResponseItemCurrency>,
    estimated_days_max: Option<i64>,
    estimated_days_min: Option<i64>,
    id: Option<String>,
    label: Option<String>,
    service_level: Option<String>,
    temperature: Option<ListShippingOptionsResponseItemTemperature>,
}

impl ListShippingOptionsResponseItemBuilder {
    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn carrier(mut self, value: impl Into<String>) -> Self {
        self.carrier = Some(value.into());
        self
    }

    pub fn currency(mut self, value: ListShippingOptionsResponseItemCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn estimated_days_max(mut self, value: i64) -> Self {
        self.estimated_days_max = Some(value);
        self
    }

    pub fn estimated_days_min(mut self, value: i64) -> Self {
        self.estimated_days_min = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn temperature(mut self, value: ListShippingOptionsResponseItemTemperature) -> Self {
        self.temperature = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListShippingOptionsResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_cents`](ListShippingOptionsResponseItemBuilder::amount_cents)
    /// - [`currency`](ListShippingOptionsResponseItemBuilder::currency)
    /// - [`id`](ListShippingOptionsResponseItemBuilder::id)
    /// - [`label`](ListShippingOptionsResponseItemBuilder::label)
    /// - [`service_level`](ListShippingOptionsResponseItemBuilder::service_level)
    /// - [`temperature`](ListShippingOptionsResponseItemBuilder::temperature)
    pub fn build(self) -> Result<ListShippingOptionsResponseItem, BuildError> {
        Ok(ListShippingOptionsResponseItem {
            amount_cents: self
                .amount_cents
                .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            carrier: self.carrier,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            estimated_days_max: self.estimated_days_max,
            estimated_days_min: self.estimated_days_min,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
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
