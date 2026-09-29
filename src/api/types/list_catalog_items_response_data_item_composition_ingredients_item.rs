pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCatalogItemsResponseDataItemCompositionIngredientsItem {
    #[serde(default)]
    pub name: String,
    pub role: ListCatalogItemsResponseDataItemCompositionIngredientsItemRole,
    #[serde(rename = "basisOfStrengthSubstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_of_strength_substance:
        Option<ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstance>,
    pub strength: ListCatalogItemsResponseDataItemCompositionIngredientsItemStrength,
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItem {
    pub fn builder() -> ListCatalogItemsResponseDataItemCompositionIngredientsItemBuilder {
        <ListCatalogItemsResponseDataItemCompositionIngredientsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemCompositionIngredientsItemBuilder {
    name: Option<String>,
    role: Option<ListCatalogItemsResponseDataItemCompositionIngredientsItemRole>,
    basis_of_strength_substance:
        Option<ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstance>,
    strength: Option<ListCatalogItemsResponseDataItemCompositionIngredientsItemStrength>,
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(
        mut self,
        value: ListCatalogItemsResponseDataItemCompositionIngredientsItemRole,
    ) -> Self {
        self.role = Some(value);
        self
    }

    pub fn basis_of_strength_substance(
        mut self,
        value: ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstance,
    ) -> Self {
        self.basis_of_strength_substance = Some(value);
        self
    }

    pub fn strength(
        mut self,
        value: ListCatalogItemsResponseDataItemCompositionIngredientsItemStrength,
    ) -> Self {
        self.strength = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemCompositionIngredientsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ListCatalogItemsResponseDataItemCompositionIngredientsItemBuilder::name)
    /// - [`role`](ListCatalogItemsResponseDataItemCompositionIngredientsItemBuilder::role)
    /// - [`strength`](ListCatalogItemsResponseDataItemCompositionIngredientsItemBuilder::strength)
    pub fn build(
        self,
    ) -> Result<ListCatalogItemsResponseDataItemCompositionIngredientsItem, BuildError> {
        Ok(ListCatalogItemsResponseDataItemCompositionIngredientsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            basis_of_strength_substance: self.basis_of_strength_substance,
            strength: self
                .strength
                .ok_or_else(|| BuildError::missing_field("strength"))?,
        })
    }
}
