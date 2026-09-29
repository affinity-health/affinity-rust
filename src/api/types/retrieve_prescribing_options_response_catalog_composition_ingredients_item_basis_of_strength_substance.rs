pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstance
{
    #[serde(default)]
    pub name: String,
}

impl RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstance {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstanceBuilder{
        <RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstanceBuilder
{
    name: Option<String>,
}

impl RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstanceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstanceBuilder::name)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstance, BuildError> {
        Ok(RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstance {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
