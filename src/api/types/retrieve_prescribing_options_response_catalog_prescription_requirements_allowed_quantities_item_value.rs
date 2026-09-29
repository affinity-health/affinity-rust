pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValue
{
    Double(f64),

        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValueOne(RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValueOne),
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_retrieve_prescribing_options_response_catalog_prescription_requirements_allowed_quantities_item_value_one(
        &self,
    ) -> bool {
        matches!(self, Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValueOne(_))
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

    pub fn as_retrieve_prescribing_options_response_catalog_prescription_requirements_allowed_quantities_item_value_one(&self) -> Option<&RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValueOne>{
        match self {
                    Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValueOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_retrieve_prescribing_options_response_catalog_prescription_requirements_allowed_quantities_item_value_one(self) -> Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValueOne>{
        match self {
                    Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValueOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display
    for RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValueOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
