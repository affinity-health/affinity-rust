pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMax {
    Double(f64),

    ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMaxOne(
        ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMaxOne,
    ),
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMax {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_catalog_items_response_data_item_prescription_requirements_quantity_increment_max_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMaxOne(
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

    pub fn as_list_catalog_items_response_data_item_prescription_requirements_quantity_increment_max_one(
        &self,
    ) -> Option<&ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMaxOne>
    {
        match self {
                    Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMaxOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_list_catalog_items_response_data_item_prescription_requirements_quantity_increment_max_one(
        self,
    ) -> Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMaxOne>
    {
        match self {
                    Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMaxOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMaxOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
