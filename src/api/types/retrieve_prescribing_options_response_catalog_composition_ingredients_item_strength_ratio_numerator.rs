pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumerator
{
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub unit: String,
}

impl RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumerator {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumeratorBuilder{
        <RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumeratorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumeratorBuilder
{
    value: Option<String>,
    unit: Option<String>,
}

impl
    RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumeratorBuilder
{
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumerator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumeratorBuilder::value)
    /// - [`unit`](RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumeratorBuilder::unit)
    pub fn build(
        self,
    ) -> Result<
        RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumerator,
        BuildError,
    > {
        Ok(RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumerator {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}
