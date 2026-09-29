pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValue {
    Double(f64),

    RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValueOne(
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValueOne,
    ),
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_retrieve_prescribing_options_response_catalog_prescription_requirements_default_quantity_value_one(
        &self,
    ) -> bool {
        matches!(self, Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValueOne(_))
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

    pub fn as_retrieve_prescribing_options_response_catalog_prescription_requirements_default_quantity_value_one(
        &self,
    ) -> Option<
        &RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValueOne,
    > {
        match self {
                    Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValueOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_retrieve_prescribing_options_response_catalog_prescription_requirements_default_quantity_value_one(
        self,
    ) -> Option<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValueOne,
    > {
        match self {
                    Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValueOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display
    for RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValueOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
