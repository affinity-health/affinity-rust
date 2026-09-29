pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemFulfillmentInclusionsItem {
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    pub kind: ListCatalogItemsResponseDataItemFulfillmentInclusionsItemKind,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "priceComponent")]
    pub price_component: ListCatalogItemsResponseDataItemFulfillmentInclusionsItemPriceComponent,
}

impl ListCatalogItemsResponseDataItemFulfillmentInclusionsItem {
    pub fn builder() -> ListCatalogItemsResponseDataItemFulfillmentInclusionsItemBuilder {
        <ListCatalogItemsResponseDataItemFulfillmentInclusionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemFulfillmentInclusionsItemBuilder {
    amount_cents: Option<i64>,
    kind: Option<ListCatalogItemsResponseDataItemFulfillmentInclusionsItemKind>,
    label: Option<String>,
    price_component:
        Option<ListCatalogItemsResponseDataItemFulfillmentInclusionsItemPriceComponent>,
}

impl ListCatalogItemsResponseDataItemFulfillmentInclusionsItemBuilder {
    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn kind(
        mut self,
        value: ListCatalogItemsResponseDataItemFulfillmentInclusionsItemKind,
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
        value: ListCatalogItemsResponseDataItemFulfillmentInclusionsItemPriceComponent,
    ) -> Self {
        self.price_component = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemFulfillmentInclusionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_cents`](ListCatalogItemsResponseDataItemFulfillmentInclusionsItemBuilder::amount_cents)
    /// - [`kind`](ListCatalogItemsResponseDataItemFulfillmentInclusionsItemBuilder::kind)
    /// - [`label`](ListCatalogItemsResponseDataItemFulfillmentInclusionsItemBuilder::label)
    /// - [`price_component`](ListCatalogItemsResponseDataItemFulfillmentInclusionsItemBuilder::price_component)
    pub fn build(
        self,
    ) -> Result<ListCatalogItemsResponseDataItemFulfillmentInclusionsItem, BuildError> {
        Ok(ListCatalogItemsResponseDataItemFulfillmentInclusionsItem {
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
        })
    }
}
