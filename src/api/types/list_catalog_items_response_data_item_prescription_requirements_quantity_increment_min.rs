pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMin {
    Double(f64),

    ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMinOne(
        ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMinOne,
    ),
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMin {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_catalog_items_response_data_item_prescription_requirements_quantity_increment_min_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMinOne(
                _
            )
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

    pub fn as_list_catalog_items_response_data_item_prescription_requirements_quantity_increment_min_one(
        &self,
    ) -> Option<&ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMinOne>
    {
        match self {
                    Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMinOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_list_catalog_items_response_data_item_prescription_requirements_quantity_increment_min_one(
        self,
    ) -> Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMinOne>
    {
        match self {
                    Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMinOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMinOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
