pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmount {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub unit: String,
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmount {
    pub fn builder(
    ) -> ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmountBuilder {
        <ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmountBuilder {
    value: Option<String>,
    unit: Option<String>,
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmountBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmountBuilder::value)
    /// - [`unit`](ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmountBuilder::unit)
    pub fn build(
        self,
    ) -> Result<
        ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmount,
        BuildError,
    > {
        Ok(
            ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmount {
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            },
        )
    }
}
