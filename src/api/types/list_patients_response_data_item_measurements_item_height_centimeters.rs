pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListPatientsResponseDataItemMeasurementsItemHeightCentimeters {
    Double(f64),

    ListPatientsResponseDataItemMeasurementsItemHeightCentimetersOne(
        ListPatientsResponseDataItemMeasurementsItemHeightCentimetersOne,
    ),
}

impl ListPatientsResponseDataItemMeasurementsItemHeightCentimeters {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_patients_response_data_item_measurements_item_height_centimeters_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ListPatientsResponseDataItemMeasurementsItemHeightCentimetersOne(_)
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

    pub fn as_list_patients_response_data_item_measurements_item_height_centimeters_one(
        &self,
    ) -> Option<&ListPatientsResponseDataItemMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::ListPatientsResponseDataItemMeasurementsItemHeightCentimetersOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_list_patients_response_data_item_measurements_item_height_centimeters_one(
        self,
    ) -> Option<ListPatientsResponseDataItemMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::ListPatientsResponseDataItemMeasurementsItemHeightCentimetersOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for ListPatientsResponseDataItemMeasurementsItemHeightCentimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListPatientsResponseDataItemMeasurementsItemHeightCentimetersOne(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}
