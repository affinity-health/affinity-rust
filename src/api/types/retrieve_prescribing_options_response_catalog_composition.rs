pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalogComposition {
    pub status: RetrievePrescribingOptionsResponseCatalogCompositionStatus,
    #[serde(default)]
    pub ingredients: Vec<RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItem>,
}

impl RetrievePrescribingOptionsResponseCatalogComposition {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogCompositionBuilder {
        <RetrievePrescribingOptionsResponseCatalogCompositionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogCompositionBuilder {
    status: Option<RetrievePrescribingOptionsResponseCatalogCompositionStatus>,
    ingredients: Option<Vec<RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItem>>,
}

impl RetrievePrescribingOptionsResponseCatalogCompositionBuilder {
    pub fn status(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogCompositionStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn ingredients(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItem>,
    ) -> Self {
        self.ingredients = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogComposition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](RetrievePrescribingOptionsResponseCatalogCompositionBuilder::status)
    /// - [`ingredients`](RetrievePrescribingOptionsResponseCatalogCompositionBuilder::ingredients)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseCatalogComposition, BuildError> {
        Ok(RetrievePrescribingOptionsResponseCatalogComposition {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            ingredients: self
                .ingredients
                .ok_or_else(|| BuildError::missing_field("ingredients"))?,
        })
    }
}
