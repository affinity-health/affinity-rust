pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemOrdering {
    #[serde(rename = "requiresPrescription")]
    #[serde(default)]
    pub requires_prescription: bool,
    #[serde(rename = "requiresAccompanyingPrescription")]
    #[serde(default)]
    pub requires_accompanying_prescription: bool,
    pub shipping: ListCatalogItemsResponseDataItemOrderingShipping,
}

impl ListCatalogItemsResponseDataItemOrdering {
    pub fn builder() -> ListCatalogItemsResponseDataItemOrderingBuilder {
        <ListCatalogItemsResponseDataItemOrderingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemOrderingBuilder {
    requires_prescription: Option<bool>,
    requires_accompanying_prescription: Option<bool>,
    shipping: Option<ListCatalogItemsResponseDataItemOrderingShipping>,
}

impl ListCatalogItemsResponseDataItemOrderingBuilder {
    pub fn requires_prescription(mut self, value: bool) -> Self {
        self.requires_prescription = Some(value);
        self
    }

    pub fn requires_accompanying_prescription(mut self, value: bool) -> Self {
        self.requires_accompanying_prescription = Some(value);
        self
    }

    pub fn shipping(mut self, value: ListCatalogItemsResponseDataItemOrderingShipping) -> Self {
        self.shipping = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemOrdering`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requires_prescription`](ListCatalogItemsResponseDataItemOrderingBuilder::requires_prescription)
    /// - [`requires_accompanying_prescription`](ListCatalogItemsResponseDataItemOrderingBuilder::requires_accompanying_prescription)
    /// - [`shipping`](ListCatalogItemsResponseDataItemOrderingBuilder::shipping)
    pub fn build(self) -> Result<ListCatalogItemsResponseDataItemOrdering, BuildError> {
        Ok(ListCatalogItemsResponseDataItemOrdering {
            requires_prescription: self
                .requires_prescription
                .ok_or_else(|| BuildError::missing_field("requires_prescription"))?,
            requires_accompanying_prescription: self
                .requires_accompanying_prescription
                .ok_or_else(|| BuildError::missing_field("requires_accompanying_prescription"))?,
            shipping: self
                .shipping
                .ok_or_else(|| BuildError::missing_field("shipping"))?,
        })
    }
}
