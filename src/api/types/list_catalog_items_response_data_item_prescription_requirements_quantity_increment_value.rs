pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValue {
    Double(f64),

    ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValueOne(
        ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValueOne,
    ),
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_catalog_items_response_data_item_prescription_requirements_quantity_increment_value_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValueOne(
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

    pub fn as_list_catalog_items_response_data_item_prescription_requirements_quantity_increment_value_one(
        &self,
    ) -> Option<&ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValueOne>
    {
        match self {
                    Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValueOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_list_catalog_items_response_data_item_prescription_requirements_quantity_increment_value_one(
        self,
    ) -> Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValueOne>
    {
        match self {
                    Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValueOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display
    for ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValueOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
