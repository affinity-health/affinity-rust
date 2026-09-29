pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCatalogItemsResponseDataItemMedicationGroup {
    #[serde(rename = "offerCount")]
    pub offer_count: ListCatalogItemsResponseDataItemMedicationGroupOfferCount,
    #[serde(rename = "pharmacyCount")]
    pub pharmacy_count: ListCatalogItemsResponseDataItemMedicationGroupPharmacyCount,
    #[serde(default)]
    pub strengths: Vec<String>,
}

impl ListCatalogItemsResponseDataItemMedicationGroup {
    pub fn builder() -> ListCatalogItemsResponseDataItemMedicationGroupBuilder {
        <ListCatalogItemsResponseDataItemMedicationGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemMedicationGroupBuilder {
    offer_count: Option<ListCatalogItemsResponseDataItemMedicationGroupOfferCount>,
    pharmacy_count: Option<ListCatalogItemsResponseDataItemMedicationGroupPharmacyCount>,
    strengths: Option<Vec<String>>,
}

impl ListCatalogItemsResponseDataItemMedicationGroupBuilder {
    pub fn offer_count(
        mut self,
        value: ListCatalogItemsResponseDataItemMedicationGroupOfferCount,
    ) -> Self {
        self.offer_count = Some(value);
        self
    }

    pub fn pharmacy_count(
        mut self,
        value: ListCatalogItemsResponseDataItemMedicationGroupPharmacyCount,
    ) -> Self {
        self.pharmacy_count = Some(value);
        self
    }

    pub fn strengths(mut self, value: Vec<String>) -> Self {
        self.strengths = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemMedicationGroup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`offer_count`](ListCatalogItemsResponseDataItemMedicationGroupBuilder::offer_count)
    /// - [`pharmacy_count`](ListCatalogItemsResponseDataItemMedicationGroupBuilder::pharmacy_count)
    /// - [`strengths`](ListCatalogItemsResponseDataItemMedicationGroupBuilder::strengths)
    pub fn build(self) -> Result<ListCatalogItemsResponseDataItemMedicationGroup, BuildError> {
        Ok(ListCatalogItemsResponseDataItemMedicationGroup {
            offer_count: self
                .offer_count
                .ok_or_else(|| BuildError::missing_field("offer_count"))?,
            pharmacy_count: self
                .pharmacy_count
                .ok_or_else(|| BuildError::missing_field("pharmacy_count"))?,
            strengths: self
                .strengths
                .ok_or_else(|| BuildError::missing_field("strengths"))?,
        })
    }
}
