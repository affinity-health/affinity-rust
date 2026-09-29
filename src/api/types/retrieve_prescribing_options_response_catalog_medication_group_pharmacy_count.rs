pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCount {
    Double(f64),

    RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCountOne(
        RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCountOne,
    ),
}

impl RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCount {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_retrieve_prescribing_options_response_catalog_medication_group_pharmacy_count_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCountOne(_)
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

    pub fn as_retrieve_prescribing_options_response_catalog_medication_group_pharmacy_count_one(
        &self,
    ) -> Option<&RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCountOne> {
        match self {
            Self::RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCountOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_retrieve_prescribing_options_response_catalog_medication_group_pharmacy_count_one(
        self,
    ) -> Option<RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCountOne> {
        match self {
            Self::RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCountOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCountOne(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
