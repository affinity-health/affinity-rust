pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListPatientsResponseDataItemClinicalProfileWeightPounds {
    Double(f64),

    ListPatientsResponseDataItemClinicalProfileWeightPoundsOne(
        ListPatientsResponseDataItemClinicalProfileWeightPoundsOne,
    ),
}

impl ListPatientsResponseDataItemClinicalProfileWeightPounds {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_patients_response_data_item_clinical_profile_weight_pounds_one(&self) -> bool {
        matches!(
            self,
            Self::ListPatientsResponseDataItemClinicalProfileWeightPoundsOne(_)
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

    pub fn as_list_patients_response_data_item_clinical_profile_weight_pounds_one(
        &self,
    ) -> Option<&ListPatientsResponseDataItemClinicalProfileWeightPoundsOne> {
        match self {
            Self::ListPatientsResponseDataItemClinicalProfileWeightPoundsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_patients_response_data_item_clinical_profile_weight_pounds_one(
        self,
    ) -> Option<ListPatientsResponseDataItemClinicalProfileWeightPoundsOne> {
        match self {
            Self::ListPatientsResponseDataItemClinicalProfileWeightPoundsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ListPatientsResponseDataItemClinicalProfileWeightPounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListPatientsResponseDataItemClinicalProfileWeightPoundsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
