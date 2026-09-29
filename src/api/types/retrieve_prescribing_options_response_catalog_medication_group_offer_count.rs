pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCount {
    Double(f64),

    RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCountOne(
        RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCountOne,
    ),
}

impl RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCount {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_retrieve_prescribing_options_response_catalog_medication_group_offer_count_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCountOne(_)
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

    pub fn as_retrieve_prescribing_options_response_catalog_medication_group_offer_count_one(
        &self,
    ) -> Option<&RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCountOne> {
        match self {
            Self::RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCountOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_retrieve_prescribing_options_response_catalog_medication_group_offer_count_one(
        self,
    ) -> Option<RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCountOne> {
        match self {
            Self::RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCountOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCountOne(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}
