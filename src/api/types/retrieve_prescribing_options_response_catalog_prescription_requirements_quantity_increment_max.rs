pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMax {
    Double(f64),

    RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMaxOne(
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMaxOne,
    ),
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMax {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_retrieve_prescribing_options_response_catalog_prescription_requirements_quantity_increment_max_one(
        &self,
    ) -> bool {
        matches!(self, Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMaxOne(_))
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

    pub fn as_retrieve_prescribing_options_response_catalog_prescription_requirements_quantity_increment_max_one(
        &self,
    ) -> Option<
        &RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMaxOne,
    > {
        match self {
                    Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMaxOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_retrieve_prescribing_options_response_catalog_prescription_requirements_quantity_increment_max_one(
        self,
    ) -> Option<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMaxOne,
    > {
        match self {
                    Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMaxOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display
    for RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMax
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMaxOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
