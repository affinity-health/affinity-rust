pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItem {
    #[serde(default)]
    pub name: String,
    pub role: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemRole,
    #[serde(rename = "basisOfStrengthSubstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_of_strength_substance: Option<
        RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstance,
    >,
    pub strength: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrength,
}

impl RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItem {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBuilder {
        <RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBuilder {
    name: Option<String>,
    role: Option<RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemRole>,
    basis_of_strength_substance: Option<
        RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstance,
    >,
    strength: Option<RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrength>,
}

impl RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemRole,
    ) -> Self {
        self.role = Some(value);
        self
    }

    pub fn basis_of_strength_substance(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBasisOfStrengthSubstance,
    ) -> Self {
        self.basis_of_strength_substance = Some(value);
        self
    }

    pub fn strength(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrength,
    ) -> Self {
        self.strength = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBuilder::name)
    /// - [`role`](RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBuilder::role)
    /// - [`strength`](RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemBuilder::strength)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItem, BuildError>
    {
        Ok(
            RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItem {
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
                basis_of_strength_substance: self.basis_of_strength_substance,
                strength: self
                    .strength
                    .ok_or_else(|| BuildError::missing_field("strength"))?,
            },
        )
    }
}
