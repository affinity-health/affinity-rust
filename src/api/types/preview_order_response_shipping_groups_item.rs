pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PreviewOrderResponseShippingGroupsItem {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub pharmacy: String,
    #[serde(default)]
    pub label: String,
    pub temperature: PreviewOrderResponseShippingGroupsItemTemperature,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(rename = "itemCount")]
    #[serde(default)]
    pub item_count: i64,
    /// Zero-based indexes into the preview prescriptions array. This is an estimated shipping charge group, not a guarantee of one physical package.
    #[serde(rename = "prescriptionIndexes")]
    #[serde(default)]
    pub prescription_indexes: Vec<i64>,
}

impl PreviewOrderResponseShippingGroupsItem {
    pub fn builder() -> PreviewOrderResponseShippingGroupsItemBuilder {
        <PreviewOrderResponseShippingGroupsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseShippingGroupsItemBuilder {
    key: Option<String>,
    pharmacy: Option<String>,
    label: Option<String>,
    temperature: Option<PreviewOrderResponseShippingGroupsItemTemperature>,
    amount_cents: Option<i64>,
    item_count: Option<i64>,
    prescription_indexes: Option<Vec<i64>>,
}

impl PreviewOrderResponseShippingGroupsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn pharmacy(mut self, value: impl Into<String>) -> Self {
        self.pharmacy = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn temperature(mut self, value: PreviewOrderResponseShippingGroupsItemTemperature) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn item_count(mut self, value: i64) -> Self {
        self.item_count = Some(value);
        self
    }

    pub fn prescription_indexes(mut self, value: Vec<i64>) -> Self {
        self.prescription_indexes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseShippingGroupsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PreviewOrderResponseShippingGroupsItemBuilder::key)
    /// - [`pharmacy`](PreviewOrderResponseShippingGroupsItemBuilder::pharmacy)
    /// - [`label`](PreviewOrderResponseShippingGroupsItemBuilder::label)
    /// - [`temperature`](PreviewOrderResponseShippingGroupsItemBuilder::temperature)
    /// - [`amount_cents`](PreviewOrderResponseShippingGroupsItemBuilder::amount_cents)
    /// - [`item_count`](PreviewOrderResponseShippingGroupsItemBuilder::item_count)
    /// - [`prescription_indexes`](PreviewOrderResponseShippingGroupsItemBuilder::prescription_indexes)
    pub fn build(self) -> Result<PreviewOrderResponseShippingGroupsItem, BuildError> {
        Ok(PreviewOrderResponseShippingGroupsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            pharmacy: self
                .pharmacy
                .ok_or_else(|| BuildError::missing_field("pharmacy"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            temperature: self
                .temperature
                .ok_or_else(|| BuildError::missing_field("temperature"))?,
            amount_cents: self
                .amount_cents
                .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            item_count: self
                .item_count
                .ok_or_else(|| BuildError::missing_field("item_count"))?,
            prescription_indexes: self
                .prescription_indexes
                .ok_or_else(|| BuildError::missing_field("prescription_indexes"))?,
        })
    }
}
