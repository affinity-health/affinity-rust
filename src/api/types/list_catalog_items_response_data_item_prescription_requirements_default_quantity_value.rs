pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValue {
    Double(f64),

    ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValueOne(
        ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValueOne,
    ),
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_catalog_items_response_data_item_prescription_requirements_default_quantity_value_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValueOne(
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

    pub fn as_list_catalog_items_response_data_item_prescription_requirements_default_quantity_value_one(
        &self,
    ) -> Option<&ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValueOne>
    {
        match self {
                    Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValueOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_list_catalog_items_response_data_item_prescription_requirements_default_quantity_value_one(
        self,
    ) -> Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValueOne>
    {
        match self {
                    Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValueOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValueOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
