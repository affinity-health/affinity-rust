pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominator {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub unit: String,
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominator {
    pub fn builder(
    ) -> ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominatorBuilder
    {
        <ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominatorBuilder
{
    value: Option<String>,
    unit: Option<String>,
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominatorBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominatorBuilder::value)
    /// - [`unit`](ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominatorBuilder::unit)
    pub fn build(
        self,
    ) -> Result<
        ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominator,
        BuildError,
    > {
        Ok(
            ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominator {
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            },
        )
    }
}
