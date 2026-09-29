pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListCatalogItemsResponseDataItemMedicationGroupPharmacyCount {
    Double(f64),

    ListCatalogItemsResponseDataItemMedicationGroupPharmacyCountOne(
        ListCatalogItemsResponseDataItemMedicationGroupPharmacyCountOne,
    ),
}

impl ListCatalogItemsResponseDataItemMedicationGroupPharmacyCount {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_catalog_items_response_data_item_medication_group_pharmacy_count_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ListCatalogItemsResponseDataItemMedicationGroupPharmacyCountOne(_)
        )
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_catalog_items_response_data_item_medication_group_pharmacy_count_one(
        &self,
    ) -> Option<&ListCatalogItemsResponseDataItemMedicationGroupPharmacyCountOne> {
        match self {
            Self::ListCatalogItemsResponseDataItemMedicationGroupPharmacyCountOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_list_catalog_items_response_data_item_medication_group_pharmacy_count_one(
        self,
    ) -> Option<ListCatalogItemsResponseDataItemMedicationGroupPharmacyCountOne> {
        match self {
            Self::ListCatalogItemsResponseDataItemMedicationGroupPharmacyCountOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for ListCatalogItemsResponseDataItemMedicationGroupPharmacyCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListCatalogItemsResponseDataItemMedicationGroupPharmacyCountOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
