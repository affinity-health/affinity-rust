pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumerator {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub unit: String,
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumerator {
    pub fn builder(
    ) -> ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumeratorBuilder
    {
        <ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumeratorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumeratorBuilder {
    value: Option<String>,
    unit: Option<String>,
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumeratorBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumerator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumeratorBuilder::value)
    /// - [`unit`](ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumeratorBuilder::unit)
    pub fn build(
        self,
    ) -> Result<
        ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumerator,
        BuildError,
    > {
        Ok(
            ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumerator {
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            },
        )
    }
}
