pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListPatientsResponseDataItemClinicalProfileHeightInches {
    Double(f64),

    ListPatientsResponseDataItemClinicalProfileHeightInchesOne(
        ListPatientsResponseDataItemClinicalProfileHeightInchesOne,
    ),
}

impl ListPatientsResponseDataItemClinicalProfileHeightInches {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_patients_response_data_item_clinical_profile_height_inches_one(&self) -> bool {
        matches!(
            self,
            Self::ListPatientsResponseDataItemClinicalProfileHeightInchesOne(_)
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

    pub fn as_list_patients_response_data_item_clinical_profile_height_inches_one(
        &self,
    ) -> Option<&ListPatientsResponseDataItemClinicalProfileHeightInchesOne> {
        match self {
            Self::ListPatientsResponseDataItemClinicalProfileHeightInchesOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_patients_response_data_item_clinical_profile_height_inches_one(
        self,
    ) -> Option<ListPatientsResponseDataItemClinicalProfileHeightInchesOne> {
        match self {
            Self::ListPatientsResponseDataItemClinicalProfileHeightInchesOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ListPatientsResponseDataItemClinicalProfileHeightInches {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListPatientsResponseDataItemClinicalProfileHeightInchesOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
