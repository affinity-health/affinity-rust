pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValue {
    Double(f64),

    RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValueOne(
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValueOne,
    ),
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_retrieve_prescribing_options_response_catalog_prescription_requirements_quantity_increment_value_one(
        &self,
    ) -> bool {
        matches!(self, Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValueOne(_))
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

    pub fn as_retrieve_prescribing_options_response_catalog_prescription_requirements_quantity_increment_value_one(
        &self,
    ) -> Option<
        &RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValueOne,
    > {
        match self {
                    Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValueOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_retrieve_prescribing_options_response_catalog_prescription_requirements_quantity_increment_value_one(
        self,
    ) -> Option<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValueOne,
    > {
        match self {
                    Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValueOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display
    for RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValueOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
