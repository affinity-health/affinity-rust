pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstance {
    #[serde(default)]
    pub name: String,
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstance {
    pub fn builder(
    ) -> ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstanceBuilder
    {
        <ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstanceBuilder
{
    name: Option<String>,
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstanceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstanceBuilder::name)
    pub fn build(
        self,
    ) -> Result<
        ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstance,
        BuildError,
    > {
        Ok(
            ListCatalogItemsResponseDataItemCompositionIngredientsItemBasisOfStrengthSubstance {
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            },
        )
    }
}
