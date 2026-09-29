pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominator
{
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub unit: String,
}

impl RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominator {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominatorBuilder{
        <RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominatorBuilder
{
    value: Option<String>,
    unit: Option<String>,
}

impl RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominatorBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominatorBuilder::value)
    /// - [`unit`](RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominatorBuilder::unit)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominator, BuildError> {
        Ok(RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominator {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}
