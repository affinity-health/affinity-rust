pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmount {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub unit: String,
}

impl RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmount {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmountBuilder{
        <RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmountBuilder
{
    value: Option<String>,
    unit: Option<String>,
}

impl
    RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmountBuilder
{
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmountBuilder::value)
    /// - [`unit`](RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmountBuilder::unit)
    pub fn build(
        self,
    ) -> Result<
        RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmount,
        BuildError,
    > {
        Ok(RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmount {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}
