pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItem {
    pub kind: RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItemKind,
    #[serde(default)]
    pub text: String,
}

impl RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItem {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItemBuilder
    {
        <RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItemBuilder {
    kind: Option<RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItemKind>,
    text: Option<String>,
}

impl RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItemBuilder {
    pub fn kind(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItemBuilder::kind)
    /// - [`text`](RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItemBuilder::text)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItem, BuildError>
    {
        Ok(
            RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItem {
                kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
                text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            },
        )
    }
}
