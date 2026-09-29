pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItem {
    pub kind: ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemKind,
    #[serde(default)]
    pub text: String,
}

impl ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItem {
    pub fn builder() -> ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemBuilder {
        <ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemBuilder {
    kind: Option<ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemKind>,
    text: Option<String>,
}

impl ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemBuilder {
    pub fn kind(
        mut self,
        value: ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemBuilder::kind)
    /// - [`text`](ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemBuilder::text)
    pub fn build(
        self,
    ) -> Result<ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItem, BuildError> {
        Ok(
            ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItem {
                kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
                text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            },
        )
    }
}
