pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItem {
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    pub kind: RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemKind,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "priceComponent")]
    pub price_component:
        RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemPriceComponent,
}

impl RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItem {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemBuilder {
        <RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemBuilder {
    amount_cents: Option<i64>,
    kind: Option<RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemKind>,
    label: Option<String>,
    price_component:
        Option<RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemPriceComponent>,
}

impl RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemBuilder {
    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn kind(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn price_component(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemPriceComponent,
    ) -> Self {
        self.price_component = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_cents`](RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemBuilder::amount_cents)
    /// - [`kind`](RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemBuilder::kind)
    /// - [`label`](RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemBuilder::label)
    /// - [`price_component`](RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemBuilder::price_component)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItem, BuildError>
    {
        Ok(
            RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItem {
                amount_cents: self
                    .amount_cents
                    .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
                kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
                label: self
                    .label
                    .ok_or_else(|| BuildError::missing_field("label"))?,
                price_component: self
                    .price_component
                    .ok_or_else(|| BuildError::missing_field("price_component"))?,
            },
        )
    }
}
